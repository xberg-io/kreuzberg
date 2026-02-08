using System.Text;
using System.Text.Json;
using Kreuzberg;

var debug = Environment.GetEnvironmentVariable("KREUZBERG_BENCHMARK_DEBUG") == "true";
var argsSpan = args.AsSpan();

// Parse OCR flag and find mode
var ocrEnabled = true;
var modeIndex = -1;

for (var i = 0; i < argsSpan.Length; i++)
{
    switch (argsSpan[i])
    {
        case "--ocr":
            ocrEnabled = true;
            break;
        case "--no-ocr":
            ocrEnabled = false;
            break;
        default:
            if (!argsSpan[i].StartsWith("--"))
            {
                modeIndex = i;
                break;
            }
            break;
    }

    if (modeIndex >= 0)
    {
        break;
    }
}

if (modeIndex < 0)
{
    Console.Error.WriteLine("Error: Mode (sync or server) is required");
    return 1;
}

var mode = argsSpan[modeIndex].ToString();

if (debug)
{
    Console.Error.WriteLine("[DEBUG] Starting C# benchmark");
    Console.Error.WriteLine($"[DEBUG] Mode: {mode}");
    Console.Error.WriteLine($"[DEBUG] OCR enabled: {ocrEnabled}");
    Console.Error.WriteLine($"[DEBUG] KREUZBERG_FFI_DIR: {Environment.GetEnvironmentVariable("KREUZBERG_FFI_DIR") ?? "(not set)"}");
    Console.Error.WriteLine($"[DEBUG] LD_LIBRARY_PATH: {Environment.GetEnvironmentVariable("LD_LIBRARY_PATH") ?? "(not set)"}");
    Console.Error.WriteLine($"[DEBUG] DYLD_LIBRARY_PATH: {Environment.GetEnvironmentVariable("DYLD_LIBRARY_PATH") ?? "(not set)"}");
    Console.Error.WriteLine($"[DEBUG] PATH: {Environment.GetEnvironmentVariable("PATH") ?? "(not set)"}");
    Console.Error.WriteLine($"[DEBUG] AppContext.BaseDirectory: {AppContext.BaseDirectory}");
}

try
{
    if (mode == "sync")
    {
        // Sync mode: extract single file and output JSON
        if (modeIndex + 1 >= argsSpan.Length)
        {
            Console.Error.WriteLine("Error: File path required for sync mode");
            return 1;
        }

        var filePath = argsSpan[modeIndex + 1].ToString();

        if (!File.Exists(filePath))
        {
            Console.Error.WriteLine($"Error: File not found: {filePath}");
            return 1;
        }

        var content = await File.ReadAllBytesAsync(filePath);
        var mimeType = GuessMimeType(filePath);

        if (debug)
        {
            Console.Error.WriteLine($"[DEBUG] File: {filePath}");
            Console.Error.WriteLine($"[DEBUG] File size: {content.Length} bytes");
            Console.Error.WriteLine($"[DEBUG] MIME type: {mimeType}");
        }

        var sw = System.Diagnostics.Stopwatch.StartNew();
        var result = KreuzbergClient.ExtractBytesSync(content, mimeType);
        sw.Stop();

        var output = new
        {
            content = result.Content,
            _extraction_time_ms = sw.Elapsed.TotalMilliseconds
        };

        var json = JsonSerializer.Serialize(output);
        Console.WriteLine(json);

        if (debug)
        {
            Console.Error.WriteLine($"[DEBUG] Extraction completed in {sw.ElapsedMilliseconds}ms");
        }

        return 0;
    }
    else if (mode == "server")
    {
        // Server mode: read file paths from stdin, extract, output JSON lines
        if (debug)
        {
            Console.Error.WriteLine("[DEBUG] Entering server mode");
        }

        // Signal readiness after .NET CLR initialization is complete
        Console.Out.WriteLine("READY");
        Console.Out.Flush();

        string? line;
        while ((line = Console.ReadLine()) != null)
        {
            if (debug)
            {
                Console.Error.WriteLine($"[DEBUG] Processing: {line}");
            }

            try
            {
                if (!File.Exists(line))
                {
                    var errorOutput = new
                    {
                        error = $"File not found: {line}",
                        _extraction_time_ms = 0.0
                    };
                    var errorJson = JsonSerializer.Serialize(errorOutput);
                    Console.WriteLine(errorJson);
                    Console.Out.Flush();
                    continue;
                }

                var content = await File.ReadAllBytesAsync(line);
                var mimeType = GuessMimeType(line);

                var sw = System.Diagnostics.Stopwatch.StartNew();
                var result = KreuzbergClient.ExtractBytesSync(content, mimeType);
                sw.Stop();

                var output = new
                {
                    content = result.Content,
                    _extraction_time_ms = sw.Elapsed.TotalMilliseconds
                };

                var json = JsonSerializer.Serialize(output);
                Console.WriteLine(json);
                Console.Out.Flush();

                if (debug)
                {
                    Console.Error.WriteLine($"[DEBUG] Successfully extracted: {line}");
                }
            }
            catch (Exception ex)
            {
                if (debug)
                {
                    Console.Error.WriteLine($"[DEBUG] Exception during extraction: {ex.GetType().Name}: {ex.Message}");
                    Console.Error.WriteLine($"[DEBUG] Full exception: {ex}");
                }

                var errorOutput = new
                {
                    error = $"{ex.GetType().Name}: {ex.Message}",
                    _extraction_time_ms = 0.0
                };
                var errorJson = JsonSerializer.Serialize(errorOutput);
                Console.WriteLine(errorJson);
                Console.Out.Flush();
            }
        }

        if (debug)
        {
            Console.Error.WriteLine("[DEBUG] Server mode: stdin closed, exiting");
        }

        return 0;
    }
    else
    {
        Console.Error.WriteLine($"Error: Unknown mode '{mode}'. Must be 'sync' or 'server'");
        return 1;
    }
}
catch (Exception ex)
{
    Console.Error.WriteLine($"Error: {ex.GetType().Name}: {ex.Message}");
    if (debug)
    {
        Console.Error.WriteLine($"[DEBUG] Full exception: {ex}");
    }
    return 1;
}

static string GuessMimeType(string path)
{
    var ext = Path.GetExtension(path).ToLowerInvariant();
    return ext switch
    {
        ".pdf" => "application/pdf",
        ".docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        ".doc" => "application/msword",
        ".pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        ".pptm" => "application/vnd.ms-powerpoint.presentation.macroEnabled.12",
        ".ppsx" => "application/vnd.openxmlformats-officedocument.presentationml.slideshow",
        ".ppt" => "application/vnd.ms-powerpoint",
        ".xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ".xlsm" => "application/vnd.ms-excel.sheet.macroEnabled.12",
        ".xlsb" => "application/vnd.ms-excel.sheet.binary.macroEnabled.12",
        ".xls" or ".xla" or ".xlam" => "application/vnd.ms-excel",
        ".odt" => "application/vnd.oasis.opendocument.text",
        ".ods" => "application/vnd.oasis.opendocument.spreadsheet",
        ".json" => "application/json",
        ".html" or ".htm" => "text/html",
        ".xml" => "application/xml",
        ".txt" => "text/plain",
        ".md" or ".markdown" or ".commonmark" => "text/markdown",
        ".csv" => "text/csv",
        ".tsv" => "text/tab-separated-values",
        ".rtf" => "application/rtf",
        ".rst" => "text/x-rst",
        ".org" => "text/x-org",
        ".latex" or ".tex" => "application/x-latex",
        ".bib" => "application/x-bibtex",
        ".epub" => "application/epub+zip",
        ".eml" => "message/rfc822",
        ".msg" => "application/vnd.ms-outlook",
        ".ipynb" => "application/x-ipynb+json",
        ".yaml" or ".yml" => "application/yaml",
        ".toml" => "application/toml",
        ".jpg" or ".jpeg" => "image/jpeg",
        ".png" => "image/png",
        ".tiff" or ".tif" => "image/tiff",
        ".gif" => "image/gif",
        ".bmp" => "image/bmp",
        ".webp" => "image/webp",
        ".jp2" => "image/jp2",
        ".jpx" => "image/jpx",
        ".jpm" => "image/jpm",
        ".mj2" => "video/mj2",
        ".svg" => "image/svg+xml",
        ".zip" => "application/zip",
        ".tar" => "application/x-tar",
        ".gz" or ".tgz" => "application/gzip",
        ".7z" => "application/x-7z-compressed",
        ".typst" or ".typ" => "application/x-typst",
        ".djot" => "text/x-djot",
        _ => "application/octet-stream",
    };
}
