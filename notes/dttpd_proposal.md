# Dttp Daemon Proposal

Author: [Kurotetsuka](github.com/kurotetsuka)  
Version: `1.0.0`  
Date: `7df210` / `2015-02-16`  
License: [MIT](
	https://github.com/kurotetsuka/hyphaelia/blob/master/legal/mit.md)  
Source: [`github.com/kurotetsuka/hyphaelia/notes/dttpd_proposal.md`](
	https://github.com/kurotetsuka/hyphaelia/blob/master/notes/dttpd_proposal.md)  

## Abstract
All our various public communication platforms are insufficient. Mailing lists are often properly signed and trustable, but do not implicitly support voting, ranking, filtering, archiving, curation, etc. Content aggregations platforms such as reddit and hacker news support ranking through voting, sometimes filtering through sub-site subscriptions, curation through moderation, but not proper digital signing methods, censorship and forgery protections, decentralization, nor web-of-trust analysis. Similarly, primitive social media platforms such as facebook and twitter are even less secure and feature rich, but provide users easy ways to connect with people or organizations they already know.

## Solution
Hyphaelia is a distributed trustable message-thread platform that implicitly supports ranking, filtering, curation, censorship protection, forgery protection, decentralization, as well as trust and respect analysis. Additionally, all of these features are theoretically optional to the end-user. Hyphaelia builds upon the concepts of pgp-signed mailing lists, adding various features inspired by common social media sites, such as voting, without the restriction of centralization that those sites impose.

## Architecture
Hyphaelia's back-end architecture consists of an overlay network of dttp daemons. These daemons collaboratively host all data stored on the network. The dttp daemons that form hyphaelia are called hubs. Hubs each store a database of units of signed communication. These units of signed communication are called motes. Motes can theoretically contain any type of data, but on the hyphaelia network, they may contain markdown text, small images, urls, respect and trust votes, and sector claims and revocations. Each hub may have zero or more named sub-hubs, called sectors. Sectors may claim any mote the hub has access to, and may later revoke that claim.

A fully featured hub daemon would be capable of many modes of operation. These modes fall under four main categories: metadata modes, which perform various operations intended to gather and serve data about motes or hubs, data modes, which perform operation on motes, analysis modes, which perform various types of analysis on stored motes, and Hyphaelia-specific extensions, which perform various operations that are only expected to be useful to the Hyphaelia network. It is not intended that every hub would attempt to fulfill every mode of operation optimally. In fact, it is intended that hubs would attempt to specialize in order provide certain services to the network and its users. The profiling metadata operation mode is included so that hubs may determine the specializations of their neighbours, and load-balance accordingly.

## Project Statement
Hyphaelia functions over a set protocols called dttp, short for distributed trustnet transmission protocol. One of the core components of the system is the dttp daemon. The goal of this project is to create a prototype dttp daemon. Additionally, a primitive client will be created to demonstrate the capabilities of the daemon.

## Requirements
The prototype daemon required by this project will be capable of the following operations modes: tracking, bootstrapping, pushing, and pulling. Additionally, fetching may be attempted. See [operation_modes.md](https://github.com/kurotetsuka/hyphaelia/blob/master/notes/operation_modes.md) for the details of these modes.
