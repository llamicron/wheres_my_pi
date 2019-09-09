# Where's my Pi?
This is a small application that sends a machines IP addresses to your discord webhook when it boots up. I use it to keep track on my raspberry Pis. They get new IP addresses every once in a while, and I hate having to go to my modem homepage or connect a keyboard and monitor just to figure out the IP so I can remote into them anyway. Now you can just restart them and it'll send you the IP.

Set up a discord webhook and put it in `~/.wheres_my_pi`. Just the webhook alone, nothing else. This is where the application will look for a webhook, so make sure it's there before you lose access.
