import React from "react";
//@ts-ignore This gets codegenerated at build time 
import docs from './docs.codegen';

export default function DocumentationPage() {
	return (<>{JSON.stringify(docs)}</>)
};