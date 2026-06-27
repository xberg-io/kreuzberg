```csharp title="simple_benchmark.cs"
using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;
using Xberg;
using System;
using System.Diagnostics;
using System.Threading.Tasks;

[MemoryDiagnoser]
[SimpleJob(warmupCount: 3, targetCount: 5)]
public class XbergBenchmark
{
    private string _testFilePath;
    private ExtractionConfig _config;

    [GlobalSetup]
    public void Setup()
    {
        _testFilePath = "document.pdf";
        _config = new ExtractionConfig
        {
            UseCache = false,
            EnableQualityProcessing = true,
        };
    }

    [Benchmark]
    public void Extract()
    {
        var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri(_testFilePath), _config)).Results[0];
        _ = result.Content.Length;
    }

    [Benchmark]
    public async Task ExtractAsync()
    {
        var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri(_testFilePath), _config)).Results[0];
        _ = result.Content.Length;
    }

    [Benchmark]
    public async Task ExtractWithOcr()
    {
        var ocrConfig = new ExtractionConfig
        {
            ForceOcr = true,
            Ocr = new OcrConfig
            {
                Backend = "tesseract",
                Language = "eng",
            }
        };

        var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri(_testFilePath), ocrConfig)).Results[0];
        _ = result.Content.Length;
    }

    [Benchmark]
    public async Task ExtractWithCache()
    {
        var cacheConfig = new ExtractionConfig
        {
            UseCache = true,
            EnableQualityProcessing = true,
        };

        var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri(_testFilePath), cacheConfig)).Results[0];
        _ = result.Content.Length;
    }
}

public class ManualBenchmark
{
    public static async Task Main(string[] args)
    {
        var filePath = "document.pdf";
        var config = new ExtractionConfig();

        (await XbergConverter.ExtractAsync(ExtractInput.FromUri(filePath), config)).Results[0];

        var sw = Stopwatch.StartNew();
        for (int i = 0; i < 10; i++)
        {
            (await XbergConverter.ExtractAsync(ExtractInput.FromUri(filePath), config)).Results[0];
        }
        sw.Stop();
        Console.WriteLine($"Sync extraction (10 runs): {sw.ElapsedMilliseconds}ms avg {sw.ElapsedMilliseconds / 10f}ms");

        sw.Restart();
        var tasks = new System.Collections.Generic.List<Task>();
        for (int i = 0; i < 10; i++)
        {
            tasks.Add(XbergConverter.ExtractAsync(ExtractInput.FromUri(filePath), config));
        }
        await Task.WhenAll(tasks);
        sw.Stop();
        Console.WriteLine($"Async extraction (10 parallel runs): {sw.ElapsedMilliseconds}ms");

        var summary = BenchmarkRunner.Run<XbergBenchmark>();
    }
}
```
