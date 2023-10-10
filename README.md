# Linear regression exploration

Exploring the [linregress](https://crates.io/crates/linregress)
and [lstsq](https://crates.io/crates/lstsq) crates and maybe others
in the future sich as [ndarray-linalg](https://crates.io/crates/ndarray-linalg)

ATM, bard.google.com couldn't get anything to compile, see
this [conversation](https://g.co/bard/share/3bace7d202c5).

ChatGPT-4 was able to get gpt4-1.rs to compile after 7 tries and then
one additional tweak was needed to get it to run. Add another data point.
Here is the [link](https://chat.openai.com/share/c2200401-7e7d-4469-a7e1-c59e8c6512ec)
to the ChatGPT-4 conversation.

So it appears that linregress only supports degree 1. It doesn't actually say that
but it does say only [here](https://docs.rs/linregress/latest/linregress/index.html)
that "only very simple formulae are supported" and doesn't seem to support
higher degree polynomials.

Add a 3rd degree polynomial fit using nalgebra in `nalgebra.rs`, this is from
[this](https://chat.openai.com/share/8cec22c2-952c-4fe4-a2fb-d5831ba793a6).
conversation with GPT-4.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
