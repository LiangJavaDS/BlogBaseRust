import styled from 'styled-components';

export const Title = styled.h1`
    &:hover {
        cursor: pointer;
        text-decoration: underline;
    }
`
export const MetaWrapper = styled.div`
    display: flex;
    font-size: 16px;
    line-height: 19px;
`
export const DateWrapper = styled.div`
    display: flex;
    align-items: center;
    color: #86909c;

    &::after {
        display: block;
        width: 2px;
        height: 16px;
        background: #e5e6eb;
        content: " ";
        margin: 0 10px;
    }
`
export const Link = styled.div`
    color: #86909c;
    cursor: text;
`