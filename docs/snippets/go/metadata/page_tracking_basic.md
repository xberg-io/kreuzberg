Package main

Import (
"fmt"
"Xberg"
)

Func main() {
cfg := xberg.ExtractionConfig{
Pages: &xberg.PageConfig{
ExtractPages: true,
},
}

    input := xberg.ExtractInputFromURI("document.pdf")
    result, err := xberg.Extract(*input, cfg)
    if err != nil {
        panic(err)
    }

    if result.Results[0].Pages != nil {
        for _, page := range result.Results[0].Pages {
            fmt.Printf("Page %d:\n", page.PageNumber)
            fmt.Printf("  Content: %d chars\n", len(page.Content))
            fmt.Printf("  Tables: %d\n", len(page.Tables))
            fmt.Printf("  Images: %d\n", len(page.Images))
        }
    }

}
