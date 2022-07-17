import styled from 'styled-components';
import { Button } from 'antd';
import { ContentCenterDiv } from '../PubComponents';

export const HeaderWrapper = styled.header`
    display: flex;
    justify-content: space-between;
    flex: 0 0 auto;
    height: 5rem;
    width: 100%;
    padding: 24px 40px;
    /* text-align: end; */
`

export const BodyWrapper = styled.div`
    margin: 0 auto;
    padding: 40px;
    flex: 1 1 auto;
    width: 960px;
    max-width: 960px;
    border-radius: 10px;
    background: linear-gradient(#E2E5FD, #F7F6FD);
`

export const FooterWrapper = styled.footer`
    height: 3rem;
    width: 100%;
`

export const BlogLabelWrapper = styled.footer`
    font-size: 21px;
    font-family: "fangsong";
    font-weight: bold;
    font-weight: 500;
`

export const AntdButton = styled(Button)`
    margin-right: 10px;
    border-radius: 5px;
`

export const FooterLink = styled(ContentCenterDiv)`
    cursor: pointer;
`