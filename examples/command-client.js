const run = async () => {
    const response = await fetch('http://0.0.0.0:3000/', {
        method: 'POST',
        body: JSON.stringify({
            input_path: './images/logo.jpeg',
            output_path: './images/logo_small.jpeg',
            width: 200,
            height: 200,
        }),
        headers: {
            'Content-Type': 'application/json',
        }
    })
    if (!response.ok) {
        const text = await response.text();
        console.error('Error:', text);
        return;
    }
};

run().catch(console.error);