import invoke from '@/utils/invoke';

export function getCountList() {
    return invoke('query_count');
}

export function getChartList() {
    return invoke('query_chart');
}
export function getOrderTotalCount() {
    return invoke('query_total_count');
}

export function fetchMonthlyPaymentSummary(year, month) {
    return invoke('fetch_monthly_payment_summary', { year, month });
}

export function fetchPaymentSummary() {
    return invoke('fetch_payment_summary');
}