# TODO

## Auth
- [ ] Password reset flow (request reset link → email token → new password)
  - use `lettre` crate with Gmail SMTP + App Password (no mail server needed)
  - reset token TTL: 60min in Redis, single use (delete on use)
- [ ] Delete account (user requests own account deletion)
