# How to install Game Engine and Smart-contracts: 

**User Match-making and Game Create**

git clone https://github.com/movechess/movechesscontract.git


**Game Engine**

git clone https://github.com/movechess/lila
  ```bash
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
