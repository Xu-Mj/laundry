import invoke from '@/utils/invoke'

export function print(items) {
  return invoke('print', {items})
}

export function setPrinter(printer) {
  return invoke('set_printer', {printer})
}

export function getSettledPrinter(printerType) {
  return invoke('get_settled_printer', {printerType})
}

export function getPrinters() {
  return invoke('get_printers')
}

export function printReceipt(data) {
  const {clothList, mount, paymentMethod, ...order} = data;

  return invoke('print_receipt', {order:{ order, mount, paymentMethod, clothes: clothList }});
}

export function printReceipt2(data) {
  const {cloths, mount, paymentMethod, ...order} = data;

  return invoke('print_receipt', {order:{ order, mount, paymentMethod, clothes: cloths }});
}
