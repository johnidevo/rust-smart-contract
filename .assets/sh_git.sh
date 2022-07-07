git add .
git status

read -p "Enter your commit message: " varname
varname=${varname:-commit}

git commit -m "$varname" #"git commit"
git remote add origin https://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@github.com/johnidevo/rust-smart-contract.git
git push origin master