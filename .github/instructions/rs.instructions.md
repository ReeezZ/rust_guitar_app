---
applyTo: '**/*.rs'
---
- Write idiomatic Rust and follow Leptos best practices for components, signals, and reactivity.
- Use Leptos view macros and component structure for UI code.
- Prefer signals for state management and avoid unnecessary clones.
- Reference the official Leptos documentation: https://docs.rs/leptos/latest/leptos/ and https://leptos.dev/docs/
  - Read in more documentation from subpages of the docs when required
  - Search the subpages even though i do not provide exact urls here, since it depends on the context which subpage should be fetched and read in.
- Always read in current documentation for leptos functions, structs, and macros when they are mentioned.
  - If a user asks about a specific Leptos function, struct, or macro, fetch the corresponding subpage from the official docs and include its content in the context.
  - If a topic is mentioned but not yet in context, search the leptos site for the relevant subpage and read it in.
- Add doc comments to public functions and components, including links to relevant Leptos docs where helpful.
- Ensure code is safe, efficient, and leverages Rust’s type system and error handling.
- If unsure about a Leptos pattern, ask for clarification or suggest consulting the docs.

## leptosfmt the formatter for leptos

This is the formatter that is being used for code within the `view!` macro. It has the following contraint:

> A note on non-doc comments
> Currently this formatter does not support non-doc comments in code blocks. It uses a fork of prettyplease for formatting rust code, and prettyplease does not support this. I would like to not diverge this fork too much (so I can easily keep in sync with upstream), therefore I didn't add non-doc comment support in my prettyplease fork for now. This means that you can use non-doc comments throughout your view macro, as long as they don't reside within code blocks.
> 
> A bit more context: prettyplease uses syn to parse rust syntax. According to https://doc.rust-lang.org/reference/comments.html#non-doc-comments non-doc comments are interpreted as a form of whitespace by the parser; syn basically ignores/skips these comments and does not include them in the syntax tree.

This means you should not use non-doc comments in the `view!` macro, but you can use them outside of it. Avoid comments within the `view!` macro, structure your code to be self documenting instead of using comments. If there is absolutely no other way to add context, you can use doc comments (///) instead.