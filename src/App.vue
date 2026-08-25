<template>
  <div class="app">
    <header class="header">
      <h1>🔒 加密工具</h1>
      <span class="badge">Rust + Vue</span>
    </header>

    <div class="main-layout">
      <!-- 左侧：控制面板 -->
      <div class="panel controls">
        <div class="section">
          <h3>模式</h3>
          <div class="radio-group">
            <label :class="{ active: mode === 'encrypt' }">
              <input type="radio" v-model="mode" value="encrypt" /> 加密
            </label>
            <label :class="{ active: mode === 'decrypt' }">
              <input type="radio" v-model="mode" value="decrypt" /> 解密
            </label>
          </div>
        </div>

        <div class="section">
          <h3>算法类型</h3>
          <select v-model="algorithm" class="select">
            <optgroup label="编码">
              <option value="base64">Base64</option>
              <option value="base64url">Base64URL</option>
              <option value="hex">Hex</option>
            </optgroup>
            <optgroup label="对称加密">
              <option value="aes">AES-256-GCM</option>
              <option value="des">DES</option>
              <option value="3des">3DES</option>
              <option value="xor">XOR</option>
            </optgroup>
            <optgroup label="哈希 (单向)">
              <option value="md5">MD5</option>
              <option value="sha256">SHA-256</option>
              <option value="sha512">SHA-512</option>
              <option value="blake2b">Blake2b</option>
              <option value="blake2s">Blake2s</option>
            </optgroup>
            <optgroup label="校验">
              <option value="crc32">CRC32</option>
            </optgroup>
          </select>
        </div>

        <div class="section" v-if="needsKey">
          <h3>密钥</h3>
          <div class="key-input">
            <input
              :type="showKey ? 'text' : 'password'"
              v-model="key"
              placeholder="输入密钥..."
              class="input"
            />
            <button @click="showKey = !showKey" class="btn-icon" :title="showKey ? '隐藏' : '显示'">
              {{ showKey ? '👁️' : '👁️‍🗨️' }}
            </button>
          </div>
        </div>

        <div class="section">
          <h3>操作</h3>
          <div class="btn-group">
            <button @click="process" :disabled="processing" class="btn primary">
              {{ processing ? '处理中...' : (isHash ? '计算哈希' : (mode === 'encrypt' ? '加密' : '解密')) }}
            </button>
            <button @click="copyResult" :disabled="!result" class="btn">
              复制结果
            </button>
            <button @click="swap" :disabled="!result" class="btn">
              结果→输入
            </button>
          </div>
        </div>

        <div class="section" v-if="result">
          <h3>状态</h3>
          <div :class="['status', success ? 'success' : 'error']">
            {{ success ? '✓ 操作成功' : ('✗ ' + errorMsg) }}
          </div>
        </div>

        <div class="section">
          <h3>文件操作</h3>
          <div class="btn-group">
            <button @click="fileEncrypt" class="btn">
              加密文件
            </button>
            <button @click="fileDecrypt" class="btn">
              解密文件
            </button>
          </div>
        </div>
      </div>

      <!-- 右侧：输入输出 -->
      <div class="panels-io">
        <div class="panel io">
          <h3>输入</h3>
          <textarea
            v-model="input"
            placeholder="在此输入文本..."
            class="textarea"
          ></textarea>
          <div class="info">{{ input.length }} 字符</div>
        </div>
        <div class="panel io">
          <h3>输出</h3>
          <textarea
            :value="result"
            readonly
            placeholder="结果将显示在这里..."
            class="textarea"
          ></textarea>
          <div class="info">{{ result.length }} 字符</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

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
    errorMsg.value = '请输入内容'
    success.value = false
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
  } catch (e) {
    result.value = ''
    success.value = false
    errorMsg.value = String(e)
  } finally {
    processing.value = false
  }
}

function copyResult() {
  if (result.value) {
    navigator.clipboard.writeText(result.value)
  }
}

function swap() {
  input.value = result.value
  result.value = ''
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
  } catch (e) {
    success.value = false
    errorMsg.value = String(e)
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
  } catch (e) {
    success.value = false
    errorMsg.value = String(e)
  }
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #1a1a2e;
  color: #e0e0e0;
  height: 100vh;
  overflow: hidden;
}

.app {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 20px;
  background: #16213e;
  border-bottom: 1px solid #0f3460;
}

.header h1 {
  font-size: 20px;
  font-weight: 600;
}

.badge {
  background: #e94560;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
}

.main-layout {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.controls {
  width: 260px;
  min-width: 260px;
  border-right: 1px solid #0f3460;
  overflow-y: auto;
  padding: 12px;
}

.panels-io {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.panel {
  background: #16213e;
  border-radius: 8px;
  margin: 8px;
}

.section {
  margin-bottom: 16px;
}

.section h3 {
  font-size: 13px;
  color: #888;
  margin-bottom: 6px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.radio-group {
  display: flex;
  gap: 8px;
}

.radio-group label {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 8px;
  background: #1a1a2e;
  border: 1px solid #0f3460;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.radio-group label.active {
  background: #0f3460;
  border-color: #e94560;
}

.radio-group input {
  display: none;
}

.select {
  width: 100%;
  padding: 8px 12px;
  background: #1a1a2e;
  color: #e0e0e0;
  border: 1px solid #0f3460;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
}

.select:focus {
  outline: none;
  border-color: #e94560;
}

.key-input {
  display: flex;
  gap: 6px;
}

.input {
  flex: 1;
  padding: 8px 12px;
  background: #1a1a2e;
  color: #e0e0e0;
  border: 1px solid #0f3460;
  border-radius: 6px;
  font-size: 14px;
}

.input:focus {
  outline: none;
  border-color: #e94560;
}

.btn-icon {
  padding: 8px;
  background: #1a1a2e;
  border: 1px solid #0f3460;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
}

.btn-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.btn {
  padding: 8px 12px;
  background: #1a1a2e;
  color: #e0e0e0;
  border: 1px solid #0f3460;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}

.btn:hover:not(:disabled) {
  background: #0f3460;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn.primary {
  background: #e94560;
  border-color: #e94560;
  font-weight: 600;
}

.btn.primary:hover:not(:disabled) {
  background: #c73650;
}

.status {
  padding: 8px 12px;
  border-radius: 6px;
  font-size: 13px;
}

.status.success {
  background: rgba(0, 200, 100, 0.15);
  color: #00c864;
  border: 1px solid rgba(0, 200, 100, 0.3);
}

.status.error {
  background: rgba(233, 69, 96, 0.15);
  color: #e94560;
  border: 1px solid rgba(233, 69, 96, 0.3);
}

.io {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 12px;
}

.io h3 {
  font-size: 13px;
  color: #888;
  margin-bottom: 6px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.textarea {
  flex: 1;
  width: 100%;
  padding: 12px;
  background: #1a1a2e;
  color: #e0e0e0;
  border: 1px solid #0f3460;
  border-radius: 6px;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 14px;
  resize: none;
}

.textarea:focus {
  outline: none;
  border-color: #e94560;
}

.textarea[readonly] {
  opacity: 0.8;
}

.info {
  margin-top: 4px;
  font-size: 12px;
  color: #666;
  text-align: right;
}
</style>
