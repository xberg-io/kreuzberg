// Hand-written binding-specific edge case tests for PDF rendering.
// Happy-path render tests are auto-generated from fixtures in e2e/.
// These tests cover error handling, validation, and lifecycle patterns
// that vary per language and can't be generated uniformly.

using System;
using System.Collections.Generic;
using System.IO;
using Xunit;

namespace Kreuzberg.Tests;

public class RenderTests : TestBase
{
    private static string GetTestPdfPath()
    {
        return NativeTestHelper.GetDocumentPath("pdf/tiny.pdf");
    }

    [Fact]
    public void RenderingMethodsExist()
    {
        Assert.NotNull(typeof(KreuzbergClient).GetMethod("RenderPdfPage"));
        Assert.NotNull(typeof(PdfPageIterator));
    }

    [Fact]
    public void RenderPdfPage_NonexistentFile_Throws()
    {
        Assert.ThrowsAny<Exception>(() =>
            KreuzbergClient.RenderPdfPage("/nonexistent/path/to/document.pdf", 0));
    }

    [Fact]
    public void RenderPdfPage_EmptyPath_ThrowsArgumentException()
    {
        Assert.Throws<ArgumentException>(() =>
            KreuzbergClient.RenderPdfPage(string.Empty, 0));
    }

    [Fact]
    public void RenderPdfPage_IndexOutOfBounds_Throws()
    {
        var path = GetTestPdfPath();

        Assert.ThrowsAny<Exception>(() =>
            KreuzbergClient.RenderPdfPage(path, 9999));
    }

    [Fact]
    public void RenderPdfPage_NegativeIndex_ThrowsArgumentOutOfRange()
    {
        var path = GetTestPdfPath();

        Assert.Throws<ArgumentOutOfRangeException>(() =>
            KreuzbergClient.RenderPdfPage(path, -1));
    }

    [Fact]
    public void PdfPageIterator_Dispose_IsSafe()
    {
        var path = GetTestPdfPath();

        var iter = PdfPageIterator.Open(path);
        iter.Dispose();
        // Double dispose should be safe
        iter.Dispose();
        // After dispose, PageCount returns 0
        Assert.Equal(0, iter.PageCount);
    }

    [Fact]
    public void PdfPageIterator_NonexistentFile_Throws()
    {
        Assert.ThrowsAny<Exception>(() =>
            PdfPageIterator.Open("/nonexistent/path/to/document.pdf"));
    }

    [Fact]
    public void PdfPageIterator_EmptyPath_ThrowsArgumentException()
    {
        Assert.Throws<ArgumentException>(() =>
            PdfPageIterator.Open(string.Empty));
    }

    [Fact]
    public void PdfPageIterator_EarlyTermination_FirstPageIsValidPng()
    {
        var path = GetTestPdfPath();

        using var iter = PdfPageIterator.Open(path);
        var enumerator = iter.GetEnumerator();
        Assert.True(enumerator.MoveNext());
        var page = enumerator.Current;
        Assert.NotNull(page);
        Assert.Equal(0, page.PageIndex);
        Assert.True(page.Data.Length > 8, "PNG data should be longer than 8 bytes");
        // PNG magic bytes: 89 50 4E 47 0D 0A 1A 0A
        Assert.Equal(0x89, page.Data[0]);
        Assert.Equal(0x50, page.Data[1]);
        Assert.Equal(0x4E, page.Data[2]);
        Assert.Equal(0x47, page.Data[3]);
        // Close without exhausting the iterator
    }
}
