// This will be available as an enum in your Python and Typescript code.
enum Category2 {
    Refund
    CancelOrder
    TechnicalSupport
    AccountIssue
    Question
}

function ClassifyMessage2(input: string) -> Category {
  client GPT4

  prompt #"
    {{ _.role("system") }}
    // You can use _.role("system") to indicate that this text should be a system message

    Classify the following INPUT into ONE
    of the following categories:

    {{ ctx.output_format }}

    {{ _.role("user") }}
    // And _.role("user") to indicate that this text should be a user message

    INPUT: {{ input }}

    Response:
  "#
}