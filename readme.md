# CIM - Composable Information Machine

We need a system of information we or our agents may compose together using all available resources.

Today that is a very different statement than it was just 10 years ago.

We come from paper. There is no reason to deny it. We have been using the idea of paper for at least 3,000 years.

The computers we use today were designed to... emit paper.

Ethernet exists to talk to... the printer.

My phone doesn't have a printer... it has a screen.

My printer never seems to want to work so it remains off most of the time.

I tend to use dashboards now, because unless you are in the widarding world, paper doesn't let photos move or words animate.

So why am I still using processes designed for paper?

CIM is a solution to that dilemna.

> "A tale of of knowledge work"

Of course we will still represent paper in the information world, though we also need to represent screens, telemetry, sensors, and storage - in both memory and persistance layers.

In the business world, we primarily work with a computer and not a phone (how long will it take for that to be dated...), but the phone is now our go to device for notifications. When I get multiple notifications, it's usually annoying. They are hard to centrally control and usually need access to our calendar... Oh, we have a bunch of those.

Tie in communications (email, sms, chat, social media, forums, virtual worlds, irc, nntp and more), applications, iot and a billion devices that now think they have a role to play in your information life, such as your refrigerator.

This is quickly overwhelming, for anyone.

Am I suposed to be the central hub for all this traffic?

That is pretty much where we are now.

Granted, some of us have everything already stuffed in hierarchical file systems and various note taking tools.

We also download files, browse the web, talk to people and 
scatter information all over, sticky notes abound.

I have most knowledge stored near the program that created it. I tend to stored groups of files in git repositories so I can also monitor changes.

I often need information I know I have, but have no idea where.
It's linked to many things, but most of those links are stored in my head and not in a well tuned system that can retrieve not only my information, but show it's relative links outside, i.e. the internet.

I have a multitude of programs available at my fingertips, but none of them really talk to each other.

I end up being the go between, and that is the classification of a knowledge worker; the person who makes all this stuff go together.

Maybe you make spreadsheets, or emails, or pictures, or video...

We all have favorite programs we use to produce our knowledge content.

Connecting those together is where CIM comes in.

It doesn't matter what programs you have or what vast amounts of data are available if they don't have context relevant to me or my business.

Vendorization can get you pretty far. We have indeed reached a point where the mega software houses are creating these tools, but they end up being very expensive and targeted at the Fortune 500 style enterprises.

If I move around from company to company it quickly becomes a massive shift in tooling, unless we stay within a vendorized segment of the market.

There has to be a better way to know store my knowledge in various partitions of iinformation that can talk to each other up in the magical cloud.

CIM fills that gap.

## CIM Architecture

We have widely available suppliers of Object Storage and if we use that with some smart ways of tracking what is in them, we have a portable factory of information we may compose to interact with an API.

Information via API. It's not a new concept, but it will replace your server storage and backups... forever. The "network", meaning the internet, was just not fast enough to do this until recently.

We use Object Storage because all I realy need to do is dump some bits somewhere...

It doesn't matter how you partition things, well it does, but we will come to that later. What I mean is, it doesn't matter what categories you partition into, that is dependent on your needs and no one elses.

The fact we do partition into categories is the real point.

Once I place something into a CIM they can't ever change. Sure I can modify them, but they turn into something else. We monitor this through a graph.

We don't dp direct manipulation in the Object Store, the Object Store is for Results and State Change Deltas.  We do use this to also hold the same idea to hold Events in an Event Store.

Now we have two "buckets" cim-objects and cim-events.

Objects are what you think of today as "files", to us, they are simply content. That content does have a shape and a structure and combining this with a representation of the information we can arrive at a Content-Address. A Content-Address is a hash that tells me what this data is, how it is shaped, what program can read it, and a graph of all the individual blocks that make up this information.

This isn't just a unique identifier, it is a code that can be replicated, but not forged.

Say I have a string and I need to store it:

Hello world.

Great, now I have this string in memory somewhere and now it needs to live in my Object Store. What happens?

First, I get a Content-Address 