"""Python deprecation helper for Kreuzberg."""

import warnings
from collections.abc import Callable
from functools import wraps
from typing import Any, TypeVar

F = TypeVar("F", bound=Callable[..., Any])


def deprecated(since: str, alternative: str | None = None, removal_version: str | None = None) -> Callable[[F], F]:
    """Decorator to mark functions as deprecated.

    Args:
        since: Version when the function was deprecated (e.g., "4.2.0")
        alternative: Recommended alternative function/method to use
        removal_version: Version when the function will be removed

    Example:
        @deprecated(since="4.2.0", alternative="new_function_name", removal_version="5.0.0")
        def old_function():
            '''This function is deprecated.'''
            pass
    """

    def decorator(func: F) -> F:
        @wraps(func)
        def wrapper(*args: Any, **kwargs: Any) -> Any:
            message = f"{func.__name__} is deprecated since {since}"
            if alternative:
                message += f". Use {alternative} instead"
            if removal_version:
                message += f". Will be removed in version {removal_version}"
            message += "."

            warnings.warn(message, category=DeprecationWarning, stacklevel=2)
            return func(*args, **kwargs)

        return wrapper  # type: ignore

    return decorator
