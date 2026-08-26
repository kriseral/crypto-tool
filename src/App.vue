<template>
  <div class="app">
    <el-container>
      <el-header class="header">
        <div class="header-left">
          <el-icon :size="22" color="#409EFF"><Lock /></el-icon>
          <span class="title">CryptoTool</span>
          <el-tag type="primary" size="small" effect="light">Rust + Vue</el-tag>
        </div>
      </el-header>

      <el-container class="main-container">
        <el-aside width="280px" class="aside">
          <el-card shadow="never" class="control-card">
            <template #header>
              <span class="card-title">控制面板</span>
            </template>

            <div class="section">
              <label class="section-label">模式</label>
              <el-radio-group v-model="mode" size="default">
                <el-radio-button value="encrypt">加密</el-radio-button>
                <el-radio-button value="decrypt">解密</el-radio-button>
              </el-radio-group>
            </div>

            <div class="section">
              <label class="section-label">算法</label>
              <el-select v-model="algorithm" style="width: 100%" size="default">
                <el-option-group label="编码">
                  <el-option label="Base64" value="base64" />
                  <el-option label="Base64URL" value="base64url" />
                  <el-option label="Hex" value="hex" />
                </el-option-group>
                <el-option-group label="对称加密">
                  <el-option label="AES-256-GCM" value="aes" />
                  <el-option label="DES" value="des" />
                  <el-option label="3DES" value="3des" />
                  <el-option label="XOR" value="xor" />
                </el-option-group>
                <el-option-group label="哈希 (单向)">
                  <el-option label="MD5" value="md5" />
                  <el-option label="SHA-256" value="sha256" />
                  <el-option label="SHA-512" value="sha512" />
                  <el-option label="Blake2b" value="blake2b" />
                  <el-option label="Blake2s" value="blake2s" />
                </el-option-group>
                <el-option-group label="校验">
                  <el-option label="CRC32" value="crc32" />
                </el-option-group>
              </el-select>
            </div>

            <div class="section" v-if="needsKey">
              <label class="section-label">密钥</label>
              <el-input
                v-model="key"
                :type="showKey ? 'text' : 'password'"
                placeholder="输入密钥"
                size="default"
              >
                <template #append>
                  <el-button @click="showKey = !showKey" :icon="showKey ? 'View' : 'Hide'" />
                </template>
              </el-input>
            </div>

            <div class="section">
              <label class="section-label">操作</label>
              <div class="btn-group">
                <el-button
                  type="primary"
                  @click="process"
                  :loading="processing"
                  style="width: 100%"
                >
                  {{ isHash ? '计算哈希' : (mode === 'encrypt' ? '加密' : '解密') }}
                </el-button>
                <el-button @click="copyResult" :disabled="!result" style="width: 100%">
                  复制结果
                </el-button>
                <el-button @click="swap" :disabled="!result" style="width: 100%">
                  结果 → 输入
                </el-button>
                <el-button type="danger" plain @click="reset" style="width: 100%">
                  重置
                </el-button>
              </div>
            </div>

            <el-alert
              v-if="result"
              :title="success ? '操作成功' : errorMsg"
              :type="success ? 'success' : 'error'"
              show-icon
              :closable="false"
              style="margin-top: 8px"
            />

            <el-divider />

            <div class="section">
              <label class="section-label">文件操作</label>
              <div class="btn-group">
                <el-button @click="fileEncrypt" style="width: 100%">
                  加密文件
                </el-button>
                <el-button @click="fileDecrypt" style="width: 100%">
                  解密文件
                </el-button>
              </div>
            </div>
          </el-card>
        </el-aside>

        <el-main class="main-content">
          <el-row :gutter="16" style="height: 100%">
            <el-col :span="12" style="height: 100%">
              <el-card shadow="never" class="io-card">
                <template #header>
                  <div class="io-header">
                    <span class="card-title">输入</span>
                    <el-text type="info" size="small">{{ input.length }} 字符</el-text>
                  </div>
                </template>
                <el-input
                  v-model="input"
                  type="textarea"
                  :autosize="false"
                  placeholder="在此输入文本..."
                  resize="none"
                />
              </el-card>
            </el-col>
            <el-col :span="12" style="height: 100%">
              <el-card shadow="never" class="io-card">
                <template #header>
                  <div class="io-header">
                    <span class="card-title">输出</span>
                    <el-text type="info" size="small">{{ result.length }} 字符</el-text>
                  </div>
                </template>
                <el-input
                  :model-value="result"
                  type="textarea"
                  :autosize="false"
                  readonly
                  placeholder="结果将显示在这里..."
                  resize="none"
                />
              </el-card>
            </el-col>
          </el-row>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Lock } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

const input = ref('')
const result = ref('')
const key = ref('')
const algorithm = ref('base64')
const mode = ref('encrypt')
const showKey = ref(false)
const processing = ref(false)
const success = ref(false)
const errorMsg = ref('')

const hashAlgorithms = ['md5', 'sha256', 'sha512', 'blake2b', 'blake2s', 'crc32']
const isHash = computed(() => hashAlgorithms.includes(algorithm.value))
const needsKey = computed(() => ['aes', 'des', '3des', 'xor'].includes(algorithm.value))

async function process() {
  if (!input.value) {
    ElMessage.warning('请输入内容')
    return
  }

  processing.value = true
  try {
    const res = await invoke('process_text', {
      input: input.value,
      key: key.value,
      algorithm: algorithm.value,
      mode: mode.value,
    })
    result.value = res.data
    success.value = res.success
    errorMsg.value = res.error || ''
    if (res.success) {
      ElMessage.success('操作成功')
    } else {
      ElMessage.error(res.error || '操作失败')
    }
  } catch (e) {
    result.value = ''
    success.value = false
    errorMsg.value = String(e)
    ElMessage.error(String(e))
  } finally {
    processing.value = false
  }
}

function copyResult() {
  if (result.value) {
    navigator.clipboard.writeText(result.value)
    ElMessage.success('已复制到剪贴板')
  }
}

function swap() {
  input.value = result.value
  result.value = ''
}

function reset() {
  input.value = ''
  result.value = ''
  key.value = ''
  success.value = false
  errorMsg.value = ''
  ElMessage.info('已重置')
}

async function fileEncrypt() {
  try {
    const filePath = await invoke('select_file')
    if (!filePath) return
    const savePath = await invoke('select_save_file')
    if (!savePath) return
    const res = await invoke('process_file', {
      filePath,
      outputPath: savePath,
      key: key.value,
      algorithm: algorithm.value,
      mode: 'encrypt',
    })
    success.value = res.success
    errorMsg.value = res.error || ''
    result.value = res.data
    if (res.success) {
      ElMessage.success('文件加密成功')
    } else {
      ElMessage.error(res.error || '文件加密失败')
    }
  } catch (e) {
    success.value = false
    errorMsg.value = String(e)
    ElMessage.error(String(e))
  }
}

async function fileDecrypt() {
  try {
    const filePath = await invoke('select_file')
    if (!filePath) return
    const savePath = await invoke('select_save_file')
    if (!savePath) return
    const res = await invoke('process_file', {
      filePath,
      outputPath: savePath,
      key: key.value,
      algorithm: algorithm.value,
      mode: 'decrypt',
    })
    success.value = res.success
    errorMsg.value = res.error || ''
    result.value = res.data
    if (res.success) {
      ElMessage.success('文件解密成功')
    } else {
      ElMessage.error(res.error || '文件解密失败')
    }
  } catch (e) {
    success.value = false
    errorMsg.value = String(e)
    ElMessage.error(String(e))
  }
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  height: 100%;
  font-family: "Helvetica Neue", Helvetica, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", Arial, sans-serif;
  background: #F2F3F5;
  color: #303133;
}

.app {
  height: 100vh;
}

.el-container {
  height: 100%;
}

.header {
  display: flex;
  align-items: center;
  background: #FFFFFF;
  border-bottom: 1px solid #DCDFE6;
  height: 56px;
  padding: 0 24px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.title {
  font-size: 18px;
  font-weight: 500;
  color: #303133;
}

.aside {
  background: #FFFFFF;
  border-right: 1px solid #DCDFE6;
  overflow-y: auto;
}

.control-card {
  border: none;
  border-radius: 0;
  height: 100%;
}

.control-card .el-card__header {
  background: #FAFAFA;
  border-bottom: 1px solid #EBEEF5;
  padding: 12px 20px;
}

.control-card .el-card__body {
  padding: 20px;
}

.card-title {
  font-size: 14px;
  font-weight: 500;
  color: #303133;
}

.section {
  margin-bottom: 20px;
}

.section-label {
  display: block;
  font-size: 14px;
  color: #606266;
  margin-bottom: 8px;
  font-weight: 500;
}

.btn-group {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.main-content {
  background: #F2F3F5;
  padding: 16px;
  overflow: hidden;
}

.io-card {
  height: calc(100vh - 104px);
  border: 1px solid #EBEEF5;
  border-radius: 4px;
}

.io-card .el-card__header {
  background: #FAFAFA;
  border-bottom: 1px solid #EBEEF5;
  padding: 12px 20px;
}

.io-card .el-card__body {
  padding: 16px;
  height: calc(100% - 48px);
  display: flex;
  flex-direction: column;
}

.io-card .el-card__body .el-textarea {
  flex: 1;
}

.io-card .el-card__body .el-textarea .el-textarea__inner {
  height: 100% !important;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  color: #303133;
  background: #FFFFFF;
  border: 1px solid #DCDFE6;
  border-radius: 4px;
  padding: 12px;
}

.io-card .el-card__body .el-textarea .el-textarea__inner:focus {
  border-color: #409EFF;
  box-shadow: 0 0 0 1px #409EFF inset;
}

.io-card .el-card__body .el-textarea .el-textarea__inner[readonly] {
  background: #FAFAFA;
  color: #606266;
}

.io-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>
