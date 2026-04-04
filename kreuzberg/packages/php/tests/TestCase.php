<?php

declare(strict_types=1);

namespace Kreuzberg\Tests;

use PHPUnit\Framework\TestCase as PHPUnitTestCase;

/**
 * Base test case for Kreuzberg tests.
 *
 * Adds convenience methods for testing error conditions.
 */
abstract class TestCase extends PHPUnitTestCase
{
    /**
     * Expect an Error to be thrown.
     *
     * This is a convenience method for expectException(Error::class).
     */
    protected function expectError(): void
    {
        $this->expectException(\Error::class);
    }
}
