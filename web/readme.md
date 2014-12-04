## project ectocat :: open source health information ##

By: [kurotetsuka](github.com/kurotetsuka)  
This work is released under the GNU GPL. See [license.md](license.md) and [gnu-gpl-v3.0.md](legal/gnu-gpl-v3.0.md) for details. )

## Development Setup
There are a few system dependencies, these are basically mariadb, ruby and graphicsmagick. On Archlinux, the following command will install them:
```bash
sudo pacman -S mariadb ruby graphicsmagick
```

There are a few gem dependencies. I didn't bother setting up a gemfile, sorry. the following command will install all the neccessary ruby gems.
```bash
gem install erubis dotenv json sinatra sinatra-contrib thin
```

To initialize the database, run `tool/db_init.sh`. The default user used by the script is `hyph`, with the password `hyph`. Make sure to start the mysql daemon beforehand.

To run the rest api, run `rake launch_api`. The rest api must be launched for much of the functionality of the app to work.

To run the web app, run `rake launch_app`. By default it runs on port 8080, so navigate to [`localhost:8080/`](http://localhost:8080/) to have a look at it.


