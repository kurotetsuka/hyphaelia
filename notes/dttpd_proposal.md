# Dttp Daemon Proposal

## Project
dttpd :: distributed trustnet transmission protocol daemon

## Author
[kurotetsuka](github.com/kurotetsuka)

## Abstract
All our various public communication platforms are insufficient. Mailing lists are often properly signed and trustable, but do not implicitly support voting, ranking, filtering, archiving, curation, etc. Content aggregations platforms such as reddit and hnews support ranking through voting, sometimes filtering through sub-site subscriptions, curation through moderation, but not proper digital signing methods, censorship and forgery protections, decentralization, nor web-of-trust analysis. Similarly, primitive social media platforms such as facebook and twitter are even less secure and feature rich, but provide users easy ways to connect with people or organizations they already know.

## Solution
Hyphaelia is a distrubuted trustable message-thread platform that implicitly supports ranking, filtering, curation, censorship protection, forgery protection, decentralization, as well as trust and respect analysis. Additionally, all of these features are theoretically optional to the end-user. Hyphaelia builds upon the concepts of pgp-signed mailing lists, adding various features inspired by common social media sites, such as voting, without the restricion of centralization that those sites impose.

## Project Statement
Hyphaelia functions over a set protocals called dttp, short for distrubuted trustnet transmission protocol. One of the core components of the system is the dttp daemon. The goal of this project is to create a prototype dttp daemon. Additionally, a primitive client will be created to demonstrate the capabilities of the daemon.

## Requirements
The prototype daemon required by this project will be capable of the following operations modes: tracking, bootstrapping, pushing, pulling, and serving. If time permits, initial implementations of the following modes may be attempted: curation, thread serving, thread ranking, and profiling. See [operation_modes.md](https://github.com/kurotetsuka/hyphaelia/blob/master/notes/operation_modes.md) for the details of these modes.

## Architecture
Hyphaelia's back-end architecture consists of an overlay network of dttp daemons. These daemons collaboratively host all data stored on the network. The dttp daemons that form hyphaelia are called hubs. Hubs store a database of units of signed communication. These units of signed communication are called motes. Motes can theoretically contain any type of data, but on the hyphaelia network, they may contain markdown text, images, urls, respect and trust votes, and sector claims and revokations. Each hub may have zero or more named sub-hubs, called sectors. Sectors may claim any mote the hub has access to, and may later revoke that claim.

A fully featured hub daemon would be capable of many modes of operation. These modes fall under four main categories: metadata modes, which perform various operations intended to gather and serve data about motes or hubs, data modes, which perform operation on motes, analysis modes, which perform various types of analysis on stored motes, and client serving modes, which perform various operations that are only expected to be useful to end-users. It is not intended that every hub would attempt to fulfill every mode of operation optimally. In fact, it is intended that hubs would attempt to specialize in order provide certain services to the network and its users. The profiling operation mode is included so that hubs may determine these specializations, and load-balance accordingly.
