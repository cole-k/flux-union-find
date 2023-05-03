# flux-union-find
This repo is a demo of [flux](https://github.com/flux-rs/flux) on the union-find algorithm.

I used [flux-snapshot](https://github.com/cole-k/flux-snapshot) while implementing
them in order to profile interesting errors. 
Refer to [flux-snapshot](./flux-snapshot) and the commit history for snapshots.
I tagged interesting commits with a short message as to why they were interesting.

Union-find is presently unproven because I tried to jump directly to proving it for
a size-based heuristic of combining trees. Making a specification for that proved
tricky. I get the feeling that it wouldn't be very hard to prove array bounds are valid
while ignoring the size bounds. But I'm more interested in the error messages I found
along the way than finishing this.
