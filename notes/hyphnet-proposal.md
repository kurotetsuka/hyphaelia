# Hyphaelia Network Proposal

Author: [Kurotetsuka](github.com/kurotetsuka)  
Version: `1.0.0`  
Date: `7df40f` / `2015-04-15`  
License: [MIT](
	https://github.com/kurotetsuka/hyphaelia/blob/master/legal/mit.md)  
Source: [`github.com/kurotetsuka/hyphaelia/notes/hyphnet_proposal.md`](
	https://github.com/kurotetsuka/hyphaelia/blob/master/notes/hyphnet_proposal.md)  

## Abstract
All our various public communication platforms are insufficient. Mailing lists are often properly signed and trustable, but do not implicitly support voting, ranking, filtering, archiving, curation, etc. Content aggregations platforms such as reddit and hacker news support ranking through voting, sometimes filtering through sub-site subscriptions, curation through moderation, but not proper digital signing methods, censorship and forgery protections, decentralization, nor web-of-trust analysis. Similarly, primitive social media platforms such as facebook and twitter are even less secure and feature rich, but provide users with easy ways to connect with people or organizations they already know.

## Solution
Hyphaelia is a distributed trustable message-thread platform that implicitly supports ranking, filtering, curation, censorship protection, forgery protection, decentralization, as well as trust and respect analysis. Additionally, all of these features are theoretically optional to the end-user. Hyphaelia builds upon the concepts of pgp-signed mailing lists, adding various features inspired by common social media sites, such as voting, without the restriction of centralization that those sites impose.

## Architecture
Hyphaelia's back-end architecture consists of an overlay network of dttp daemons. These daemons collaboratively host all data stored on the network. The dttp daemons that form hyphaelia are called hubs. Hubs each store a database of units of signed communication. These units of signed communication are called motes. Motes can theoretically contain any type of data, but on the hyphaelia network, they may contain markdown text, small images, urls, respect and trust votes, and sector claims and revocations. Each hub may have zero or more named sub-hubs, called sectors. Sectors may claim any mote the hub has access to, and may later revoke that claim.

 - user respect and trust votes
 - content respect votes

## Project Statement
 - hyphd - hyphaelia-specialized hub daemon
 - hyphic - user cli front end

## Requirements
