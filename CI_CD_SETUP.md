# 🚀 CI/CD Setup - Guia Passo a Passo

Este guia vai te ajudar a configurar o CI/CD no GitHub para seu projeto.

## ✅ Checklist

- [ ] Criar repositório no GitHub
- [ ] Fazer push do código
- [ ] Configurar Codecov
- [ ] Adicionar secrets no GitHub
- [ ] Testar o CI/CD
- [ ] Atualizar badges no README

---

## 📋 Passo 1: Criar Repositório no GitHub

1. Acesse https://github.com/new
2. **Nome do repositório**: `rust-yt-downloader`
3. **Descrição**: "A professional CLI tool for downloading YouTube videos, built in Rust"
4. **Visibilidade**: Public (para usar GitHub Actions grátis)
5. **NÃO** marque "Initialize with README" (já temos um)
6. Clique em **"Create repository"**

---

## 📋 Passo 2: Fazer Push do Código

No terminal, dentro do diretório do projeto:

```bash
cd /Users/guilherme.canella/Documents/rust-yt-downloader

# Inicializar git (se ainda não fez)
git init

# Adicionar todos os arquivos
git add .

# Fazer primeiro commit
git commit -m "Initial commit: Complete YouTube downloader with CI/CD"

# Adicionar remote (substitua SEU_USUARIO pelo seu username do GitHub)
git remote add origin https://github.com/SEU_USUARIO/rust-yt-downloader.git

# Renomear branch para main (se necessário)
git branch -M main

# Push para o GitHub
git push -u origin main
```

**⚠️ IMPORTANTE**: Substitua `SEU_USUARIO` pelo seu username do GitHub!

---

## 📋 Passo 3: Configurar Codecov (Opcional mas Recomendado)

1. Acesse https://codecov.io/
2. Faça login com sua conta GitHub
3. Clique em **"Add new repository"**
4. Selecione `rust-yt-downloader`
5. Copie o **token** que aparecer
6. Vá para: https://github.com/SEU_USUARIO/rust-yt-downloader/settings/secrets/actions
7. Clique em **"New repository secret"**
8. **Name**: `CODECOV_TOKEN`
9. **Value**: Cole o token do Codecov
10. Clique em **"Add secret"**

---

## 📋 Passo 4: Adicionar Token do Crates.io (Para Releases)

**Isso é necessário apenas se quiser publicar automaticamente no crates.io quando criar uma tag.**

1. Acesse https://crates.io/me
2. Faça login com GitHub
3. Vá em **"Account Settings" → "API Tokens"**
4. Clique em **"New Token"**
5. **Nome**: `GitHub Actions - rust-yt-downloader`
6. Copie o token gerado
7. Vá para: https://github.com/SEU_USUARIO/rust-yt-downloader/settings/secrets/actions
8. Clique em **"New repository secret"**
9. **Name**: `CARGO_TOKEN`
10. **Value**: Cole o token do crates.io
11. Clique em **"Add secret"**

---

## 📋 Passo 5: Atualizar README com seu Username

Abra o arquivo `README.md` e substitua todas as ocorrências de `SEU_USUARIO` pelo seu username do GitHub.

**Procure por**:
- `https://github.com/SEU_USUARIO/rust-yt-downloader`
- Badges no topo do arquivo

**Substitua por** (exemplo):
- `https://github.com/guilhermecanella/rust-yt-downloader`

---

## 📋 Passo 6: Testar o CI/CD

Depois do push, o CI/CD vai rodar automaticamente!

1. Vá para: https://github.com/SEU_USUARIO/rust-yt-downloader/actions
2. Você verá o workflow **"CI"** rodando
3. Clique nele para ver os detalhes
4. Aguarde a conclusão (pode levar 5-10 minutos)

**O que o CI/CD vai fazer**:
- ✅ Rodar testes em Linux, macOS e Windows
- ✅ Verificar formatação do código
- ✅ Rodar Clippy (linter)
- ✅ Gerar code coverage
- ✅ Fazer security audit
- ✅ Compilar documentação
- ✅ Build de release

---

## 📋 Passo 7: Criar uma Release (Quando Pronto)

Para criar uma release e publicar no crates.io:

```bash
# 1. Atualizar a versão no Cargo.toml (exemplo: 0.1.0 → 0.2.0)
# 2. Commit a mudança
git add Cargo.toml
git commit -m "Bump version to 0.2.0"

# 3. Criar uma tag
git tag -a v0.2.0 -m "Release v0.2.0"

# 4. Push da tag
git push origin v0.2.0
```

Isso vai:
- ✅ Criar uma GitHub Release automaticamente
- ✅ Compilar binários para Linux, macOS e Windows
- ✅ Publicar no crates.io (se configurou o CARGO_TOKEN)

---

## 🎨 Badges do README

Depois que o CI/CD rodar pela primeira vez, os badges vão funcionar:

- **CI Badge**: Mostra se os testes estão passando
- **Codecov Badge**: Mostra a cobertura de código
- **Crates.io Badge**: Mostra a versão publicada (após publicar)
- **Docs.rs Badge**: Link para a documentação

---

## 🐛 Troubleshooting

### "CI está falhando"
- Verifique os logs em: https://github.com/SEU_USUARIO/rust-yt-downloader/actions
- Clique no workflow que falhou
- Expanda os steps para ver o erro
- Geralmente é:
  - Testes falhando (rode `cargo test` localmente)
  - Formatação (rode `cargo fmt`)
  - Clippy warnings (rode `cargo clippy`)

### "Codecov badge não aparece"
- Aguarde o primeiro CI rodar completamente
- Verifique se adicionou o `CODECOV_TOKEN` nos secrets
- O badge pode levar alguns minutos para atualizar

### "Release não está publicando no crates.io"
- Verifique se adicionou o `CARGO_TOKEN` nos secrets
- Certifique-se de que criou a tag com `v` no início (exemplo: `v0.1.0`)
- O nome do crate precisa estar disponível no crates.io

---

## 📚 Recursos Adicionais

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Codecov Documentation](https://docs.codecov.com/)
- [Crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)

---

## ✅ Checklist Final

Antes de publicar no crates.io, verifique:

- [ ] Todos os testes passando no CI
- [ ] Coverage badge verde
- [ ] README atualizado com badges funcionais
- [ ] CHANGELOG.md criado (opcional mas recomendado)
- [ ] Licença MIT incluída
- [ ] Versão no Cargo.toml correta
- [ ] Documentação completa
- [ ] Nome do crate disponível no crates.io

---

**Pronto! Seu projeto agora tem CI/CD profissional! 🎉**

Se tiver qualquer dúvida, me pergunte!
