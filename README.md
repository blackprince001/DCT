# DCT

Simple Data Consumption Tracker (could have a system information scrapper tool).

Scrape Network Data information and make analytics out of it.
Supposed to be an installable that gives you and endpoint to monitor how much traffic you have on your device.

Things to do:

- [X] Scrape network data
- [ ] Possible CLI interaction?
- [X] Web socket and an endpoint to send stuff.
- [ ] Minimal webpage to vizualize info from web server.

There Metrics are shared via states, and states are defined by the current info we have about the network interface scraped.
We create simple endpoints to send these metrics to and that's all that is happening. Some of the metrics attributes were added by AI, make things more expressive and those their added logic. Everything is being computed from bytes sent and received.

Contributions are more than welcomed. Deep Dives would shared on my blog if you want to know how this works?
