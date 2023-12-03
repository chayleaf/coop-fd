# coop-ofd

This is a simple accounting server for me and my roommates to keep track
of our shared purchases. It integrates with a single Russian OFD
(Operator of Fiscal Data) to allow automatically adding receipts using
QR codes (they are mandatory on receipts in Russia, but there are
like 15 OFDs, so this may not work in your area).

It will probably break in the future as it doesn't use any official
APIs, and I'll only fix it if I still use this by then.

Some of the features don't have a UI, because I wrote a
[maubot](https://github.com/maubot/maubot) plugin for using these.

## License

AGPL-3.0-or-later

Some third-party JS libraries are bundled, see
[static/README.md](static/README.md)
