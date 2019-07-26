const express = require('express');
const bodyParser = require('body-parser');

const app = express();

app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

const data = {
    1: {
        data: "Hello",
    },
};

app.get('/', (_, res) => {
    const ret = Object.keys(data).reduce((acc, item) => {
        acc.push({ id: item, ...data[item] });
        return acc;
    }, []);
    res.json(ret);
});

app.post('/', (req, res) => {
    if (req.body.id) {
        delete req.body.id;
    }
    data[req.query.id] = req.body;
    res.send('');
});

app.delete('/', (req, res) => {
    delete data[req.query.id];
    res.send('');
});

const PORT = Number(process.env.PORT) || 8080;

(async () => {
    app.listen(PORT, () => console.log(`Application Server running on port ${PORT}`));
})();