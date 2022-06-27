import React, { FC, useCallback } from "react";
import { Form, Input, Button, message } from 'antd';
import { LoginParams } from './type';
import apiClient from '@/utils/axios';

type LoginFormProps = {}
/** 登录 */
const LoginForm: FC<LoginFormProps> = () => {

    const [form] = Form.useForm();

    const login = useCallback(async () => {
        const formItemObj = await form.validateFields()
        if (!formItemObj) return
        const { username, password, } = formItemObj as LoginParams
        if (!username || !password) return
        apiClient
            .post("login", { username, password })
            .then((res) => {
                message.success("登录成功");
                if (res.data.token) {
                    localStorage.setItem('token', res.data.token);
                    console.log('7878res', res);
                }
            })
            .catch((err) => {
            });
        // const data = await postRequest("login", { username, password })
        // if (!data) return
        // message.success("登录成功");
    }, [])

    return <div>
        <Form form={form} name="basic">
            <Form.Item
                label="用户名"
                name="username"
                rules={[{ required: true, message: '请输入用户名!' }]}
            >
                <Input />
            </Form.Item>
            <Form.Item
                label="密码"
                name="password"
                rules={[{ required: true, message: '请输入密码!' }]}
            >
                <Input />
            </Form.Item>
        </Form>
        <Button onClick={login}>登录</Button>
    </div>
}
export default LoginForm