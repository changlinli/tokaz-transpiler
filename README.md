# A Transpiler for Illustrating Feasibility of Dependently Typed Language

This is project for developing a transpiler for a hypothetical dependently typed language called "Tokaz."

The basic idea behind Tokaz is that unlike other dependently typed languages such as Idris or Agda, but more similar to languages such as Coq (e.g. its division between `Prop` and `Set`), is to "not take the Curry-Howard Isomorphism too seriously." That is to maintain a division between how the language treats proofs and programs for improved DX reasons, even though from the viewpoint of programming language theory the two could be unified.

Indeed, taking things even further than Coq, the ideal is to have all computation done entirely with non-dependent types and reserve dependent types for proofs with no computational content.

The hope is that this creates a language which is:

+ Quick to compile (shooting for compilation speeds on par with Pascal of around a million lines per second)
+ Able to leverage LLMs and other AI-assisted methods or at least not purely human-deductive methods for generating proofs.
+ Lower bar to entry for programmers from mainstream programming languages

The main things we are trading off to make this happen are:

+ More cumbersome proofs: every proof written in another dependently typed language should be translatable to Tokaz, but it might be very unwieldy
+ Little to no ad-hoc polymorphism: in service of both a lower bar to entry and compilation speed, nearly all programming affordances that involve some sort of implicit argument passing or implicit instance resolution will be eschewed. This means that ad-hoc polymorphism will be likely non-existent.
+ Generally more tolerance for code verbosity
