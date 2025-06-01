Ravenloft Ritual 2 Companion
----------------------------

This is to help solve the ritual we are currently facing.  After installing rust, you can run this by doing the following:

```bash
cargo run
```

I didn't get around to properly painting the symbols, so everything is showing up as a peace sign right now.  Thus what should have been rendering the symbol on the right, I'm instead outputting the symbol's description.  The symbols are ordered by occurance, and hovering over a symbol gives some basic info about it.  There is no persistence (I haven't gotten around to it yet), so changing settings or configuring symbol values will not carry over between program executions.


Usage
=====

Clicking either a symbol (on a pillar) or a description (on the left hand side) will open a modal to configure it.  Letters used by other symbols will be unavailable to be selected.
