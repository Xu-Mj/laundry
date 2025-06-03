import invoke from '@/utils/invoke';

export function getCountList() {
    return invoke('query_count');
}

export function getChartList(params) {
    return invoke('query_chart', params);
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