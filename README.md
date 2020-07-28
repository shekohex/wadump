<h1 align="center">WAdump</h1>
<div align="center">
  <strong>
    A CLI Tool to Analyze WhatsApp Web Packets
  </strong>
</div>

<br />

## About

This tool helps in rev-eng WhatsApp Protocol and How the client is implemented.

Example:

```bash
$ wadump --outgoing -m M0VCMDhCMUY5QzdCMDUxN0ExREYsEIBQ1W3KQtF6rJgTb3S81zDddfUUUBz7MNfCswrCXBgJQw/P10FLJvjucpPx9U7yUt2FA8aaY/8jvWDTItse0nOu1jRRjBDLuTb5JW8pUZoX7pvN0CeGeszd44v/+RG+JNbKbh1MDo7V9nPAYfTcJk4b23Hr8eHWpp3w+odw3fSXAk7fEB/mbL3vqZGTaDJdXbkst89oQSfZBu9Ynk344uan
```

```rust
WebMessageInfo {
    key: MessageKey {
        remote_jid: Some(
            "[OMItTED]@s.whatsapp.net",
        ),
        from_me: Some(
            true,
        ),
        id: Some(
            "3EB08B1F9C7B0517A1DF",
        ),
        participant: None,
    },
    message: Some(
        Message {
            conversation: Some(
                "Hey, I\'m using WhatsApp 😀",
            ),
            sender_key_distribution_message: None,
            image_message: None,
            contact_message: None,
            location_message: None,
            extended_text_message: None,
            document_message: None,
            audio_message: None,
            video_message: None,
            call: None,
            chat: None,
            protocol_message: None,
            contacts_array_message: None,
            highly_structured_message: None,
            fast_ratchet_key_sender_key_distribution_message: None,
            send_payment_message: None,
            live_location_message: None,
            request_payment_message: None,
            decline_payment_request_message: None,
            cancel_payment_request_message: None,
            template_message: None,
            sticker_message: None,
            group_invite_message: None,
            template_button_reply_message: None,
            product_message: None,
            device_sent_message: None,
            device_sync_message: None,
        },
    ),
    message_timestamp: Some(
        1595891552,
    ),
    status: Some(
        Pending,
    ),
    participant: None,
    ignore: None,
    starred: None,
    broadcast: None,
    push_name: None,
    media_ciphertext_sha256: None,
    multicast: None,
    url_text: None,
    url_number: None,
    message_stub_type: None,
    clear_media: None,
    message_stub_parameters: [],
    duration: None,
    labels: [],
    payment_info: None,
    final_live_location: None,
    quoted_payment_info: None,
    ephemeral_start_timestamp: None,
    ephemeral_duration: None,
}
```

## Install

1. Easy way

```bash
$ curl -LSfs https://raw.githubusercontent.com/shekohex/wadump/master/install.sh | sh -s -- -f
```

> in WSL it fallback to windows

2. Github Releases [here](https://github.com/shekohex/wadump/releases)

3. Or if you want to build it localy

```bash
$ git clone https://github.com/shekohex/wadump.git
$ cd wadump
```

```bash
$ cargo run -- --help
```

## Usage

```bash
Usage: wadump [-r] [--outgoing] [-m <message>]

CLI Tool to dump WhatsApp Packets

Options:
  -r, --reset       reset the saved session and clear the keys
  --outgoing        is this message outgoing? (i.e you are the sender)
  -m, --message     a base64 message to be decrypted and dumped, if not provided
                    it will try to read it from stdin
  --help            display usage information
```

In the first usage, it will ask you about the `Encryption Key` and `MAC Key` and will be stored in your OS Keychain, if you don't want to use the Keychain
you could also set the `WADUMP_ENC_KEY` and `WADUMP_MAC_KEY` environment variables and it will use them instead (even if there are keys stored in the keychain)

## FAQ

1. How do I get my `Encryption Key` and `MAC Key`?

- Well, that is super easy open [WhatsApp Web](https://web.whatsapp.com/) then open the Browser Developer Tools (Hit F12)
  And then from there go to the `Application` tab and select `Storage` > `Local Storage` and select the `WASecretBundle`.
  Copy the `encKey` and `macKey` values and paste them when the program prompt for them, they are stored localy in your OS Keychain.

2. How to View Ongoing and Outgoing Messages?

- If you read FAQ #1 you alrady know how to open the Browser Developer Tools, now go to `Network` tab and Filter by `WS` (i.e Websocket)
  if it is not visable, try to refersh the page and you will see a ws entry, click on it from there you could click on the `Messages` tab.
  now you could see the incomming and the outgoing messages.
  Click in anyone of them and then by default it will view it as Hex Dump, change the `Hex Viewer` value to `Base64` and there will be a little copy icon too.

3. How could I ...

- Open an Issue :)

## Contributing

Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[contributing]: https://github.com/shekohex/wadump/blob/master/.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/shekohex/wadump/labels/good%20first%20issue
[help-wanted]: https://github.com/shekohex/wadump/labels/help%20wanted

## Safety

This crate uses `#![deny(unsafe_code)]` to ensure everything is implemented in
100% Safe Rust.

## License

<sup>
Licensed under <a href="LICENSE"> GPL-3.0 license</a>.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the GPL-3.0 license, without any additional terms or conditions.
</sub>

## Legal

<sup>
This code is in no way affiliated with, authorized, maintained, sponsored or endorsed by WhatsApp or any of its affiliates or subsidiaries. This is an independent and unofficial software. Use at your own risk.
</sup>
