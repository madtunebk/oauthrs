<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{{ code }} - {{ title }}</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: sans-serif;
            background: #0f1117;
            color: #e0e0e0;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
            text-align: center;
            padding: 2rem;
        }
        h1 { font-size: 6rem; color: #fff; margin-bottom: 0.5rem; }
        h2 { font-size: 1.5rem; color: #aaa; margin-bottom: 1rem; }
        p  { color: #666; margin-bottom: 2rem; }
        a  { color: #4a90e2; text-decoration: none; }
        a:hover { opacity: 0.8; }
    </style>
</head>
<body>
    <h1>{{ code }}</h1>
    <h2>{{ title }}</h2>
    <p>{{ message }}</p>
    <a href="/">← Go home</a>
</body>
</html>
