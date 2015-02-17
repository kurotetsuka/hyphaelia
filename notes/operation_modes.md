# Operation modes of the dttp daemon
Any subset of these operation modes may be enabled at either initialization or run time. Some modes do depend on the data produced from others, so not every possible combination may be practical. For example, in order to pull a desired mote, the daemon must have some tracking information. It would, however, be possible to enable tracking just long enough to find the desired mote, then disable it. These choices are the responsibility of the administrator.

Todo:  
 - add sector handling stuff


## Metadata modes
These modes all perform various operations intended to gather data about motes or hubs.

### Tracking
 - Keeping track of who (probably) has certain motes.

### Bootstrapping
 - Discovering other hubs, keeping track of them, detecting when hubs leave the network, etc.
 - Assisting other's hub discovery process by serving above information.

### Profiling
 - Benchmarking other hubs.

## Data modes
These modes all perform operations on motes.

### Pushing
 - Giving our new data to other hubs.

### Pulling
 - Grabbing new or desired motes off of other hubs.

### Fetching
 - Searching through the network for a specific desired mote.

### Serving
 - Responding to pushes, pulls and fetches.


## Analysis modes
These modes all perform various types of analysis on gathered motes.

### Curation
Hub owners may wish to remove junk, spam, or old motes from their servers. Additionally, some hubs may need to respond to legal takedown requests. Thus, automatic parameter-based curation may be desired. While this could theoretically enable censorship, the distributed nature of the system makes such censorship an optional, democrative process.


## Other modes

### Pollination
 - Assisting initial mote distribution process.
	 - informing trackers of new mote
	 - seeding : pushing to initial hub(s)
	 - dispersion : hubs pushing to other hubs

### Daisy-chain serving
 - Acting as a step in an indirect pull.

### Onion serving
 - Acting as a step in onion routing of pulls.


## Hyphaelia extensions
These modes are all solely intended to serve data to end-users of the Hyphaelia network. They are not expected to be useful to other dttp networks nor other hubs on the Hyphaelia network.

### Encrypted mote serving
Users may wish to distribute a mote to certain other users, and only them. There are two ways to do this - encrypted mote serving is one and private hubs are the other. In the case of encrypted mote serving, the hub stores the unencrypted mote, and only serves copies that are encrypted for authorized target(s). This implicitly requires the source user to trust the serving hub.

## Hyphaelia extensions
These modes are all specific to Hyphaelia. Most of them are 

### Thread serving
Compilation of entire mote-threads for user's perusal.

### Thread ranking
Ranking of mote-threads according to user's preferences and respect rankings. This requires some knowledge of the user, and thus registration with the hub.

### Trust analysis
asdf

### Respect analysis
asdf
