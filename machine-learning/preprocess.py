import numpy as np
import pandas as pd
from sklearn.preprocessing import LabelEncoder, OneHotEncoder
from sklearn.impute import SimpleImputer

dataset = pd.read_csv('Data.csv')
X = dataset.iloc[:,:-1].values
Y = dataset.iloc[:,-1].values
print(dataset)

imputer = SimpleImputer()
X[:,1:3] = imputer.fit_transform(X[:,1:3])
print(X)
X.fillna(X.mode(), inplace=True)
print(X)
# labelencoder_X = LabelEncoder()
# X[:,0] = labelencoder_X.fit_transform(X[:,0])
# print(X)


# onehotencoder  = OneHotEncoder(categorical_features=[0])
# X = onehotencoder.fit_transform(X).toarray()
# labelencoder_Y = LabelEncoder()
# Y = labelencoder_Y.fit_transform(Y)
# print(X)
# print(Y)