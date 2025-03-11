import invoke from '@/utils/invoke'

export function print(items) {
  return invoke('print', {items})
}

export function setPrinter(printer) {
  return invoke('set_printer', {printer})
}

export function getSettledPrinter() {
  return invoke('get_settled_printer')
}

export function getPrinters() {
  return invoke('get_printers')
}
