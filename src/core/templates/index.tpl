<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{{ app_name }}</title>
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
        }
        h1   { font-size: 3rem; color: #fff; margin-bottom: 0.5rem; }
        p    { color: #888; margin-bottom: 2rem; font-size: 1.1rem; }
        .badge {
            display: inline-block;
            padding: 0.25rem 0.75rem;
            border-radius: 999px;
            font-size: 0.75rem;
            font-weight: bold;
            margin-bottom: 2rem;
            background: #1e3a2f;
            color: #4caf7d;
        }
        .links a {
            display: inline-block;
            margin: 0.5rem;
            padding: 0.75rem 2rem;
            border-radius: 8px;
            text-decoration: none;
            font-weight: bold;
            font-size: 1rem;
            transition: opacity 0.2s;
        }
        .links a:hover { opacity: 0.8; }
        .login  { background: #4a90e2; color: #fff; }
        .signup { background: #1e1e2e; color: #aaa; border: 1px solid #333; }
        footer  { margin-top: 3rem; color: #444; font-size: 0.8rem; }
    </style>
</head>
<body>
    <span class="badge">{{ env }} mode</span>
    <h1>{{ app_name }}</h1>
    <p>Secure OAuth 2.0 authorization server</p>
    <div class="links">
        <a class="login"  href="/login">Login</a>
        <a class="signup" href="/signup">Request Access</a>
    </div>
    <footer>OauthRS &mdash; running on {{ host }}:{{ port }}</footer>
</body>
</html>
