const express = require('express')
const fs = require('fs')

const app = express()
const port = 3000

app.get('/static/js/main.*', (req, res) => {
  fs.readdir(`${__dirname}/build/static/js`, (err, files) => {
    if (err) {
      console.log("Main js bundle not created, check yarn build in Dockerfile")
    } else {
      const fileName = files.find(file => file.match(/main\..+\.js/))
      
      res.sendFile(`${__dirname}/build/static/js/${fileName}`)
    }
  })
  
})

app.listen(port, () => {
  console.log(`Nova UI bundle serving on port ${port}`)
})