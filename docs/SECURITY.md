# Web TOTP 安全性分析

## 数据存储方式

### 1. 文件存储位置
```
data.enc  # 所有数据加密保存在这个文件中
```

### 2. 存储的数据内容

#### JSON 结构（加密前）
```json
{
  "user": {
    "username": "admin",
    "password_hash": "密码哈希值（PBKDF2-HMAC-SHA256）",
    "two_fa_enabled": true/false,
    "two_fa_secret": "Base32密钥（如：ABCD1234...）"
  },
  "totp_entries": [
    {
      "id": "uuid",
      "name": "Google",
      "issuer": "Google",
      "secret": "Base32密钥",
      "created_at": "2025-10-22T..."
    }
  ]
}
```

## 安全机制

### 🔒 1. 密码保护（PBKDF2）

**实现**:
```rust
pub fn hash_password(password: &str) -> String {
    use pbkdf2::pbkdf2;
    use sha2::Sha256;
    use hmac::Hmac;
    
    const SALT: &[u8] = b"web-totp-salt";  // 固定盐值
    const ITERATIONS: u32 = 100_000;        // 10万次迭代
    
    let mut hash = [0u8; 32];
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), SALT, ITERATIONS, &mut hash);
    hex::encode(hash)  // 转为十六进制字符串存储
}
```

**安全性**:
- ✅ **单向哈希**: 无法从哈希值反推原密码
- ✅ **密钥派生**: 使用 PBKDF2 标准算法
- ✅ **多次迭代**: 10万次 HMAC-SHA256，增加暴力破解成本
- ⚠️ **固定盐值**: 所有用户使用相同盐值（安全隐患）

**攻击难度**:
- 暴力破解时间（8位强密码）: ~数年到数十年（取决于算力）
- 彩虹表攻击: 由于固定盐值，可能有效

### 🔐 2. 文件加密（XOR）

**实现**:
```rust
const ENCRYPTION_KEY: &[u8; 32] = b"web-totp-encryption-key-32bytes!";

fn encrypt_data(data: &AppData) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let json = serde_json::to_string(data)?;
    let mut encrypted = json.as_bytes().to_vec();
    
    // 简单 XOR 加密
    for (i, byte) in encrypted.iter_mut().enumerate() {
        *byte ^= ENCRYPTION_KEY[i % ENCRYPTION_KEY.len()];
    }
    
    Ok(encrypted)
}
```

**安全性**:
- ⚠️ **XOR 加密**: 这是**演示级别**的加密，不安全
- ⚠️ **硬编码密钥**: 密钥直接写在代码中
- ⚠️ **无随机性**: 相同输入产生相同输出
- ⚠️ **易被破解**: 通过已知明文攻击可轻易破解

**攻击难度**:
- 有源码: **几秒钟**（直接读取密钥）
- 无源码但有经验: **几分钟**（XOR加密模式识别）
- 已知明文攻击: **瞬间**（JSON格式可预测）

## 🚨 安全风险评估

### 高风险 🔴

#### 1. XOR "加密"根本不安全
**问题**:
```
明文: {"user":{"username":"admin"...
密钥: web-totp-encryption-key-32bytes!
加密: 明文 XOR 密钥
```

**破解方法**:
```python
# 已知 JSON 格式，可以推导密钥
known_plaintext = '{"user":{'
encrypted_bytes = read_file('data.enc')[:len(known_plaintext)]
key = [p ^ e for p, e in zip(known_plaintext, encrypted_bytes)]
# 得到密钥后可以解密全部内容
```

#### 2. 密钥硬编码在源码中
```rust
const ENCRYPTION_KEY: &[u8; 32] = b"web-totp-encryption-key-32bytes!";
```
- 任何人查看源码就能看到密钥
- 无法更改密钥（除非重新编译）

#### 3. 固定盐值
```rust
const SALT: &[u8] = b"web-totp-salt";
```
- 所有用户使用相同盐值
- 可以预计算彩虹表
- 降低密码哈希安全性

### 中风险 🟡

#### 4. 2FA 密钥明文存储（加密后）
- 2FA 密钥以明文形式存在 JSON 中
- 虽然文件加密了，但加密方式不安全
- 一旦文件被解密，所有 2FA 密钥泄露

#### 5. 无完整性校验
- 没有 HMAC 或签名
- 攻击者可以修改 data.enc 而不被发现
- 可能导致数据篡改

### 低风险 🟢

#### 6. 密码哈希（PBKDF2）相对安全
- 10万次迭代使暴力破解困难
- 使用标准算法 PBKDF2-HMAC-SHA256
- 只要用户密码强度够，相对安全

## 实际破解场景

### 场景 1: 攻击者获得 data.enc 文件

**步骤**:
1. 打开 web-totp 源码
2. 找到加密密钥: `b"web-totp-encryption-key-32bytes!"`
3. 写一个 Python 脚本:
```python
import json

ENCRYPTION_KEY = b"web-totp-encryption-key-32bytes!"

with open('data.enc', 'rb') as f:
    encrypted = f.read()

decrypted = bytearray()
for i, byte in enumerate(encrypted):
    decrypted.append(byte ^ ENCRYPTION_KEY[i % len(ENCRYPTION_KEY)])

data = json.loads(decrypted.decode('utf-8'))
print("用户名:", data['user']['username'])
print("密码哈希:", data['user']['password_hash'])
print("2FA密钥:", data['user']['two_fa_secret'])
print("TOTP条目:", data['totp_entries'])
```

**结果**: 
- ⏱️ **破解时间**: 5分钟（写脚本的时间）
- 🔓 **获得**: 所有 2FA 密钥（明文）
- ⚠️ **风险**: 攻击者可以生成所有账户的 2FA 验证码

### 场景 2: 攻击者只有 data.enc，没有源码

**步骤**:
1. 观察文件开头几个字节
2. 猜测是 JSON 格式（`{"user"`）
3. 使用已知明文攻击:
```python
known = b'{"user":'
encrypted_start = encrypted_data[:len(known)]
key_start = bytes([k ^ e for k, e in zip(known, encrypted_start)])
# 推导出密钥的前几个字节
# 根据模式推导完整密钥
```

**结果**:
- ⏱️ **破解时间**: 30分钟-2小时
- 🔓 **难度**: 中等（需要了解 XOR 加密）

### 场景 3: 暴力破解密码

**即使获得密码哈希**:
```
哈希: e4b3c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
算法: PBKDF2-HMAC-SHA256
迭代: 100,000次
盐值: "web-totp-salt"
```

**破解时间**（取决于密码强度）:
- 弱密码（如 "123456"）: 几分钟
- 中等密码（如 "admin123"）: 几小时-几天
- 强密码（如 "Xk9#mP2$vQ8!"）: 数年-数十年

## 🛡️ 改进建议

### 立即改进（Critical）

#### 1. 使用真正的加密算法
```rust
// 替换 XOR 为 AES-256-GCM
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};

// 使用随机 nonce
let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
```

#### 2. 使用环境变量或密钥派生
```rust
// 不要硬编码
const ENCRYPTION_KEY: &[u8; 32] = b"web-totp-encryption-key-32bytes!"; // ❌

// 使用用户密码派生密钥
let key = pbkdf2_derive_key(user_password, &unique_salt); // ✅
```

#### 3. 每用户独立盐值
```rust
// 固定盐值 ❌
const SALT: &[u8] = b"web-totp-salt";

// 随机盐值 ✅
let salt = SaltString::generate(&mut OsRng);
```

### 中期改进（Important）

#### 4. 添加完整性校验
```rust
// 添加 HMAC 防篡改
let hmac = Hmac::<Sha256>::new_from_slice(&key)?;
hmac.update(&ciphertext);
let tag = hmac.finalize();
```

#### 5. 使用密钥管理服务
```rust
// 考虑使用操作系统密钥链
// Windows: DPAPI
// macOS: Keychain
// Linux: Secret Service API
```

### 长期改进（Nice to have）

#### 6. 数据库存储
- 使用 SQLite/PostgreSQL
- 利用数据库加密功能
- 更好的权限控制

#### 7. 硬件安全模块
- TPM 芯片
- YubiKey
- 硬件加密

## 📊 当前安全等级评分

| 项目 | 评分 | 说明 |
|------|------|------|
| 密码存储 | 6/10 | PBKDF2 好，但固定盐值 |
| 文件加密 | 2/10 | XOR 不安全，密钥硬编码 |
| 2FA密钥保护 | 2/10 | 依赖不安全的文件加密 |
| 完整性保护 | 0/10 | 无任何完整性校验 |
| 密钥管理 | 1/10 | 硬编码在源码中 |
| **总体评分** | **2.2/10** | ⚠️ **仅适合学习/演示** |

## 🎯 适用场景

### ✅ 适合
- 个人学习项目
- 本地测试环境
- 概念验证（PoC）
- 了解 TOTP 工作原理

### ❌ 不适合
- 生产环境
- 存储敏感数据
- 多用户环境
- 互联网暴露服务

## 🔐 生产级改进示例

```rust
// 使用 AES-256-GCM 加密
use aes_gcm::{Aes256Gcm, Key, Nonce};
use argon2::{Argon2, PasswordHash, PasswordHasher};
use rand::rngs::OsRng;

// 1. 密码哈希（带随机盐）
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

// 2. 文件加密（AES-256-GCM）
fn encrypt_data(data: &AppData, user_password: &str) -> Result<Vec<u8>> {
    // 从用户密码派生加密密钥
    let key = derive_key(user_password)?;
    let cipher = Aes256Gcm::new(&key);
    
    // 生成随机 nonce
    let nonce = Nonce::from_slice(&random_nonce());
    
    // 加密
    let json = serde_json::to_string(data)?;
    let ciphertext = cipher.encrypt(nonce, json.as_bytes())?;
    
    // 返回: nonce + ciphertext
    Ok([nonce.as_slice(), &ciphertext].concat())
}

// 3. 密钥派生
fn derive_key(password: &str) -> Result<[u8; 32]> {
    let salt = load_or_generate_salt()?;
    let mut key = [0u8; 32];
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), &salt, 600_000, &mut key);
    Ok(key)
}
```

## 📝 总结

### 当前实现的安全性

**优点**:
- ✅ 密码使用 PBKDF2 哈希
- ✅ 10万次迭代增加破解难度
- ✅ 文件不是明文存储

**缺点**:
- ❌ XOR "加密"形同虚设
- ❌ 密钥硬编码在源码
- ❌ 固定盐值降低安全性
- ❌ 无完整性校验
- ❌ 无密钥管理

### 破解难度

| 条件 | 破解时间 | 难度 |
|------|---------|------|
| 有源码 + data.enc | 5分钟 | 极易 |
| 仅 data.enc（有密码学知识） | 30分钟-2小时 | 简单 |
| 仅密码哈希（强密码） | 数年 | 困难 |

### 建议

**对于当前版本**:
- ⚠️ 仅用于学习和本地测试
- ⚠️ 不要存储真实的敏感 2FA 密钥
- ⚠️ 不要暴露到互联网

**对于生产使用**:
- 必须实现 AES-256-GCM 或 ChaCha20-Poly1305
- 必须使用随机盐值和 nonce
- 必须使用环境变量或密钥管理服务
- 建议使用 Argon2id 代替 PBKDF2
- 建议添加完整性校验（HMAC/GCM tag）

---

**免责声明**: 本工具的当前加密实现仅供学习目的，不适合保护真实的敏感数据。

