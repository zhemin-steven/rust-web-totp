# API 文档

Web TOTP RESTful API 完整文档

## 基础信息

- **Base URL**: `http://127.0.0.1:18007/api`
- **Content-Type**: `application/json`
- **认证方式**: Cookie-based Session

---

## 数据库管理

### 解锁数据库

**端点**: `POST /unlock`

**描述**: 使用主密码解锁加密数据库

**请求体**:
```json
{
  "master_password": "your-master-password"
}
```

**响应**:
```json
{
  "success": true,
  "message": "Database unlocked successfully"
}
```

**错误响应**:
```json
{
  "success": false,
  "message": "Invalid master password"
}
```

---

### 获取锁定状态

**端点**: `GET /lock-status`

**描述**: 检查数据库是否已解锁

**响应**:
```json
{
  "locked": false
}
```

---

## 用户认证

### 检查用户 2FA 状态

**端点**: `POST /check-user-2fa`

**描述**: 检查指定用户是否启用了 2FA

**请求体**:
```json
{
  "username": "admin"
}
```

**响应**:
```json
{
  "requires_2fa": true
}
```

---

### 登录

**端点**: `POST /login`

**请求体**:
```json
{
  "username": "admin",
  "password": "your-password",
  "totp_code": "123456"  // 可选，启用2FA时需要
}
```

**成功响应**:
```json
{
  "success": true,
  "message": "Login successful",
  "requires_2fa": null
}
```

**需要 2FA 响应**:
```json
{
  "success": false,
  "message": "2FA code required",
  "requires_2fa": true
}
```

---

### 登出

**端点**: `POST /logout`

**响应**:
```json
{
  "success": true,
  "message": "Logged out successfully"
}
```

---

### 检查会话

**端点**: `GET /check-session`

**响应**:
```json
{
  "success": true,
  "message": "Authenticated"
}
```

---

### 修改密码

**端点**: `POST /change-password`

**认证**: 需要登录

**请求体**:
```json
{
  "old_password": "current-password",
  "new_password": "new-password"
}
```

**响应**:
```json
{
  "success": true,
  "message": "Password changed successfully"
}
```

---

## 2FA 设置

### 启用 2FA

**端点**: `POST /enable-2fa`

**认证**: 需要登录

**响应**:
```json
{
  "secret": "ABCD1234EFGH5678",
  "qr_code": "data:image/svg+xml;base64,...",
  "otpauth_url": "otpauth://totp/WebTOTP:admin?secret=...&issuer=WebTOTP"
}
```

---

### 验证并确认 2FA

**端点**: `POST /verify-2fa`

**认证**: 需要登录

**请求体**:
```json
{
  "code": "123456"
}
```

**响应**:
```json
{
  "success": true,
  "message": "2FA enabled successfully"
}
```

---

### 禁用 2FA

**端点**: `POST /disable-2fa`

**认证**: 需要登录

**请求体**:
```json
{
  "password": "current-password",
  "code": "123456"
}
```

**响应**:
```json
{
  "success": true,
  "message": "2FA disabled successfully"
}
```

**安全性**: 需要当前密码和 2FA 验证码双重验证

---

### 获取 2FA 状态

**端点**: `GET /2fa-status`

**认证**: 需要登录

**响应**:
```json
{
  "enabled": true
}
```

---

## TOTP 管理

### 添加 TOTP 条目

**端点**: `POST /totp/add`

**认证**: 需要登录

**请求体**:
```json
{
  "name": "Google",
  "issuer": "Google",
  "secret": "JBSWY3DPEHPK3PXP"
}
```

**响应**:
```json
{
  "id": "uuid-1234",
  "name": "Google",
  "issuer": "Google",
  "secret": "JBSWY3DPEHPK3PXP",
  "created_at": "2025-10-22T..."
}
```

---

### 获取 TOTP 列表

**端点**: `GET /totp/list`

**认证**: 需要登录

**响应**:
```json
[
  {
    "id": "uuid-1234",
    "name": "Google",
    "issuer": "Google",
    "secret": "JBSWY3DPEHPK3PXP",
    "created_at": "2025-10-22T..."
  }
]
```

---

### 删除 TOTP 条目

**端点**: `POST /totp/delete`

**认证**: 需要登录

**请求体**:
```json
{
  "id": "uuid-1234"
}
```

**响应**:
```json
{
  "success": true,
  "message": "Entry deleted successfully"
}
```

---

### 生成验证码

**端点**: `GET /totp/generate/{id}`

**认证**: 需要登录

**路径参数**:
- `id`: TOTP 条目的 UUID

**响应**:
```json
{
  "code": "123456",
  "remaining_seconds": 25
}
```

---

## 错误响应

### 标准错误格式

```json
{
  "success": false,
  "message": "Error description"
}
```

### HTTP 状态码

| 状态码 | 说明 |
|--------|------|
| 200 | 成功或业务逻辑错误 |
| 401 | 未认证 |
| 503 | 数据库锁定 |
| 500 | 服务器内部错误 |

### 常见错误

| 错误信息 | 原因 |
|---------|------|
| "Database is locked" | 数据库未解锁 |
| "Not authenticated" | 未登录或会话过期 |
| "Invalid master password" | 主密码错误 |
| "Invalid password" | 登录密码错误 |
| "Invalid 2FA code" | 2FA 验证码错误 |
| "Entry not found" | TOTP 条目不存在 |

---

## 数据模型

### User
```typescript
{
  username: string,
  password_hash: string,
  two_fa_enabled: boolean,
  two_fa_secret: string | null
}
```

### TotpEntry
```typescript
{
  id: string,         // UUID
  name: string,       // 账户名称
  issuer: string,     // 发行者
  secret: string,     // Base32 密钥
  created_at: string  // ISO 8601 时间戳
}
```

---

## 认证流程

### 完整登录流程

```
1. POST /check-user-2fa
   → 检查用户是否启用 2FA
   
2. 如果启用 2FA:
   POST /login (with username, password, totp_code)
   
   如果未启用:
   POST /login (with username, password only)
   
3. 登录成功 → 设置会话 Cookie
   
4. 后续请求自动带上 Cookie

5. POST /logout → 清除会话
```

---

## 使用示例

### JavaScript 示例

```javascript
// 解锁数据库
async function unlockDatabase(masterPassword) {
  const response = await fetch('/api/unlock', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ master_password: masterPassword })
  });
  return await response.json();
}

// 登录
async function login(username, password, totpCode = null) {
  const response = await fetch('/api/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ 
      username, 
      password, 
      totp_code: totpCode 
    })
  });
  return await response.json();
}

// 添加 TOTP 账户
async function addTotpEntry(name, issuer, secret) {
  const response = await fetch('/api/totp/add', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ name, issuer, secret })
  });
  return await response.json();
}

// 生成验证码
async function generateCode(entryId) {
  const response = await fetch(`/api/totp/generate/${entryId}`);
  return await response.json();
}
```

---

## 安全注意事项

1. **主密码**: 永远不要通过网络传输或记录
2. **会话**: Cookie 使用 HttpOnly 和 Secure 标志
3. **2FA 密钥**: 所有密钥加密存储在 data.enc
4. **日志**: 敏感信息不会记录在日志中

---

**作者**: Steven  
**版本**: v2.0.0  
**许可**: MIT

