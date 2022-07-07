git add .
git status

read -p "Enter your commit message: " varname
varname=${varname:-commit}

git commit -m "$varname" #"git commit"
git remote add origin https://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@github.com/johnidevo/rust-smart-contract.git
git push origin master