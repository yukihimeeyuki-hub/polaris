import { invoke } from '@tauri-apps/api/core'

interface RequestConfig {
  url: string
  method: string
  headers?: Record<string, string>
  body?: string
  query?: Record<string, string>
  timeout?: number
}

interface ResponseData {
  status: number
  status_text: string
  headers: Record<string, string>
  body: string
}

class HttpClient {
  async request(config: RequestConfig): Promise<ResponseData> {
    return invoke('http_request', { config })
  }

  async get(url: string): Promise<ResponseData> {
    return invoke('http_get', { url })
  }

  async post(url: string, body: string): Promise<ResponseData> {
    return invoke('http_post', { url, body })
  }

  async put(url: string, body: string): Promise<ResponseData> {
    return invoke('http_put', { url, body })
  }

  async delete(url: string): Promise<ResponseData> {
    return invoke('http_delete', { url })
  }

  async head(url: string): Promise<ResponseData> {
    return invoke('http_head', { url })
  }

  async options(url: string): Promise<ResponseData> {
    return invoke('http_options', { url })
  }
  async get_with_config(config: RequestConfig):Promise<ResponseData>{
    return invoke('http_get_option', { config })
  }
}

export const http = new HttpClient()
export type { RequestConfig, ResponseData }
