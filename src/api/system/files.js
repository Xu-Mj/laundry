import invoke from '@/utils/invoke'

export function uploadFile(name, data) {
    return invoke('save_file', { name, data })
  }
  
  export function deleteFile(filePath) {
    return invoke('delete_file', { filePath })
  }
  