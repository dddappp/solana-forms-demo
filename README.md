# READ ME

This article describes how to develop a Solana form program (i.e., smart contract).

This project is inspired by [Move Forms](https://github.com/dddappp/aptos-forms-demo).

## Init

Use Anchor CLI to init a workspace:

```shell
anchor init solana-forms-demo
cd solana-forms-demo
```

## ~~Programming~~

We will use the visual form designer to "design" the form program, so we don't actually need to program here. But for now we do need to install some CLI tools.

So before getting started, you need to:

* Install [Rust & Anchor & Solana & Yarn](https://www.anchor-lang.com).
* Install [Docker](https://docs.docker.com/engine/install/).

### How to design a form and get its schema

XRender is a form solution open-sourced by Alibaba. It supports designing and rendering forms and submitting form data in JSON format to backends.
It was originally a Web2 solution. But obviously we can improve it and make it work for Web3.

You can use this Form Schema Builder to design a form: https://xrender.fun/schema-builder-online

You can use the *Export (导出)* button to export the form's schema to the clipboard, then save the exported schema as a plain text file.

### Run dddappp Project Creation Tool

We use the XRender form schema file `form-schema/form.json` to generate the DDDML model file. The generated model file is available at `dddml/forms.yaml`.

> **Tip**
>
> About DDDML, here is an introductory article: ["Introducing DDDML: The Key to Low-Code Development for Decentralized Applications"](https://github.com/wubuku/Dapp-LCDP-Demo/blob/main/IntroducingDDDML.md).

In repository root directory, run:

```shell
docker run \
-v .:/myapp \
wubuku/dddappp-solana:0.0.1 \
--xRenderFormSchema /myapp/form-schema/form.json \
--xRenderFormId SolanaFormsDemo \
--dddmlDirectoryPath /myapp/dddml \
--boundedContextName Test.SolanaFormsDemo \
--solanaProgramProjectDirectoryPath /myapp/programs/solana-forms-demo \
--solanaProgramId DJYREhvzFLcicT1aaqgsSigLkj8v3vwiiWKaPyPFTcos \
--solanaBoundedContextMod solana_forms_demo \
--boundedContextJavaPackageName org.test.solanaformsdemo \
--javaProjectsDirectoryPath /myapp/solana-java-service \
--javaProjectNamePrefix solanaformsdemo \
--pomGroupId test.solanaformsdemo
```

The command parameters above are straightforward:

* The first line indicates mounting your local directory into the `/myapp` directory inside the container.
* `xRenderFormSchema` is the path of XRender form schema file. It should be a readable file path in the container.
* `xRenderFormId` is the ID of XRender form. It's recommended to use PascalCase naming style.
* `dddmlDirectoryPath` is the directory where DDDML model files are located. It should be a readable directory path in the container.
* Interpret the value of (optional) parameter `boundedContextName` as the name of your application you want to develop. When there are multiple parts in your name, separate them with dots and use PascalCase naming style for each part. Bounded-context is a term in Domain-driven design (DDD) that refers to a specific problem domain scope that contains specific business boundaries, constraints, and language. If you don't understand this concept for now, it's not a big deal.
* `solanaProgramProjectDirectoryPath` is directory path where on-chain Solana program project is placed. It should be a readable and writable directory path in container.
* `solanaProgramId` is Solana program Id. You can find it in `Anchor.toml`.
* (Optional) `solanaBoundedContextMod` is the name of bounded context module name in Solana program. It's recommended to use snake_case naming style.
* (Optional) `boundedContextJavaPackageName` is Java package name of off-chain service. According to Java naming conventions, it should be all lowercase and parts should be separated by dots.
* `javaProjectsDirectoryPath` is directory path where off-chain service code is placed. Off-chain service consists of multiple modules (projects). It should be a readable and writable directory path in container.
* (Optional) `javaProjectNamePrefix` is name prefix of each module of off-chain service. It's recommended to use an all-lowercase name.
* (Optional) `pomGroupId` is GroupId of off-chain service. We use Maven as project management tool for off-chain service. It should be all lowercase and parts should be separated by dots.

If you don't specify the optional parameters, the tool will derive them based on `xRenderFormId`.

Currently the tool only generates on-chain contracts, so Java-related parameters are not actually used.

## Test Example

Now you can test it like a regular Anchor Solana program. You realize that you didn't even write a single line of code😄.

Run local single-node dev cluster:

```shell
solana-test-validator
```

Build & deploy program:

```shell
anchor build && anchor deploy
```

Stream transaction logs in another terminal:

```shell
solana logs --url localhost
```

Run TypeScript test:

```shell
# Of course, this still requires you to modify the TS test file like this repo first
anchor run test
```

## Tips

### Update dddappp Docker Image

Since the dddappp v0.0.1 image is updated frequently, you may be required to manually delete the image and pull it again before `docker run`.

```shell
# If you have already run it, you may need to Clean Up Exited Docker Containers first
docker rm $(docker ps -aq --filter "ancestor=wubuku/dddappp-solana:0.0.1")
# remove the image
docker image rm wubuku/dddappp-solana:0.0.1
# pull the image
git pull wubuku/dddappp-solana:0.0.1
```
