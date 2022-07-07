git add .
git status

read -p "Enter your commit message: " varname
varname=${varname:-commit}

git commit -m "$varname" #"git commit"
git remote add origin https://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@github.com/johnidevo/rust-smart-contract.git
git push origin master