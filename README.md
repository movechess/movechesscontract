# How to install Game Engine and Smart-contracts: 

# localhost app simulate: http://127.0.0.1:8080/

**User Match-making and Game Create**

git clone https://github.com/movechess/movechesscontract.git


**Game Engine**

git clone https://github.com/movechess/lila
  ```bash
Tools and dependency managers
git
java (JDK >= 17, type java --version to see if you need to update or use jenv if you need multiple jdk versions)
cs (cs >= 2, installing coursier will provide sbt as well)
node (node >= 14.3, install via NVM preferred) or (install node only)
pnpm (pnpm 8.x, type npm i -g pnpm after installing node) or (see the pnpm homepage)
Running infrastructure
mongodb (5.1 >= mongo >= 4.2, instructions, WSL2)
For WSL2, you might want to manually create the default /data/db directory and give ownership permissions to your user (sudo chown -R `id -un` /data/db). If sudo service mongod start does not work, you may want to open a terminal and run mongod as super-user.
redis

cd lila
mongosh lichess < bin/mongodb/indexes.js # creates database indexes
# or `mongosh mongodb://localhost:27017/lichess < lila/bin/mongodb/indexes.js` if you use docker
ui/build # builds client. -h for help and -w for incremental watch mode.
./lila # starts the SBT console

run
```
**MoveChess Auth Server**: 

git clone https://github.com/movechess/movechess-auth-sv.git
  ```bash 
  npm install
  npm start
```

**Chess-Simulate**
```bash 
  git clone https://github.com/movechess/chess-simulate.git
  npm run build
  npm run serve
```

# localhost app simulate: http://127.0.0.1:8080/

