const fs = require('fs');
const { Readable } = require('stream');
const { finished } = require('stream/promises');

const filePath = './images/logo.jpeg';
const url = 'http://0.0.0.0:3000/?width=200&height=200&extension=jpeg';

fs.readFile(filePath, async (err, data) => {
    if (err) {
        console.error('Error reading file:', err);
        return;
    }

    const blob = new Blob([data], { type: 'image/jpeg' });

    const formData = new FormData();
    formData.append('file', blob, 'logo_small.jpeg');

    const response = await fetch(url, {
        method: 'POST',
        body: formData,
    })
    if (!response.ok) {
        const text = await response.text();
        console.error('Error:', text);
        return;
    }
    const fileWriteStream = fs.createWriteStream('./images/logo_small.jpeg');
    await finished(Readable.fromWeb(response.body).pipe(fileWriteStream));
});