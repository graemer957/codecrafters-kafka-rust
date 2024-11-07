[![progress-banner](https://backend.codecrafters.io/progress/kafka/3428f866-7bff-4475-8962-3e6790aba58b)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Kafka" Challenge](https://codecrafters.io/challenges/kafka).

In this challenge, you'll build a toy Kafka clone that's capable of accepting
and responding to APIVersions & Fetch API requests. You'll also learn about
encoding and decoding messages using the Kafka wire protocol. You'll also learn
about handling the network protocol, event loops, TCP sockets and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

# Passing the first stage

The entry point for your Kafka implementation is in `src/main.rs`. Study and
uncomment the relevant code, and push your changes to pass the first stage:

```sh
git commit -am "pass 1st stage" # any msg
git push origin master
```

That's all!

# Stage 2 & beyond

Note: This section is for stages 2 and beyond.

1. Ensure you have `cargo (1.82)` installed locally
1. Run `./your_program.sh` to run your Kafka broker, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
1. Commit your changes and run `git push origin master` to submit your solution
   to CodeCrafters. Test output will be streamed to your terminal.

# Section Notes

## Send Correlation ID (NV3)

* [Kafka wire protocol](https://kafka.apache.org/protocol.html)
* Kafka header versions: [KIP-482](https://cwiki.apache.org/confluence/display/KAFKA/KIP-482%3A+The+Kafka+Protocol+should+Support+Optional+Tagged+Fields)
    - [SO Answer](https://stackoverflow.com/a/71853003)
* Validate locally using:
    `echo -n "Placeholder request" | nc -v localhost 9092 | hexdump -C`

## Parse CorrelationID (WA6)

* [Protocol Primitive Types](https://kafka.apache.org/protocol.html#protocol_types)
* Validate locally using:
    `echo -n "00000023001200046f7fc66100096b61666b612d636c69000a6b61666b612d636c6904302e3100" | xxd -r -p | nc localhost 9092 | hexdump -C`
* I needed to use `shutdown` on the `stream`, otherwise the CI tests failed and would like to dig further into this

## Parse API Version (NC5)

* Validate locally using:
    `echo -n "00000023001200046f7fc66100096b61666b612d636c69000a6b61666b612d636c6904302e3100" | xxd -r -p | nc localhost 9092 | hexdump -C`
* Refactored work done to date to make use of `Bytes`
