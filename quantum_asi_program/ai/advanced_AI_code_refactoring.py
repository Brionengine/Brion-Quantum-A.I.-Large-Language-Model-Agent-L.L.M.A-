from transformers import pipeline  # type: ignore


def ai_refactor_code(code_snippet: str) -> str:
    """Refactor code using an AI model."""
    refactoring_pipeline = pipeline(
        "text-generation", model="codeparrot/code-generation"
    )
    prompt = (
        "Refactor the following Python code to improve readability and efficiency:\n"
        f"{code_snippet}\n"
    )
    refactored_code = refactoring_pipeline(
        prompt, max_length=512, num_return_sequences=1
    )[0]["generated_text"]
    return refactored_code


def refactor_code(code_snippet: str) -> str:
    """Return the AI-refactored version of ``code_snippet``."""
    return ai_refactor_code(code_snippet)


__all__ = ["refactor_code"]
