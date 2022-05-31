import { defineConfig } from 'umi';
import routes from './routes';

/** umi第二种配置文件，根目录下的.umirc.ts具有更高的优先级，所以.umirc.ts已被删除 */
export default defineConfig({
    nodeModulesTransform: {
        type: 'none',
    },
    routes: routes,
    fastRefresh: {},
    // 加速热更新，针对dev，这么配，原理是使用了webpack5的Module Federation打包提速方案，原理是将应用的依赖构建为一个Module Federation的remote应用，免去热更新时对依赖的编译
    mfsu: {},
    // 如果需要针对生产环境生效，这么配
    // mfsu: { production: { output: '.mfsu-production' } },
    webpack5: {}
});