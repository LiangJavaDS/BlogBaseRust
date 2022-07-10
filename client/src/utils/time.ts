import moment from "moment";

/** 格式化时间 */
export const formatTime = (time: string) => {
    if (!time) return time
    // 秒
    const diffTime = moment().diff(time) / 1000;
    if (diffTime <= 59) return "刚刚"
    if (diffTime < 60 * 60) return `${Math.floor(diffTime / 60)}分钟前`
    if (diffTime < 24 * 60 * 60) return `${Math.floor(diffTime / 60 / 60)}小时前`
    if (diffTime < 30 * 24 * 60 * 60) return `${Math.floor(diffTime / 24 / 60 / 60)}天前`
    return moment(time).format('YYYY--MM-DD hh:mm:ss');
}