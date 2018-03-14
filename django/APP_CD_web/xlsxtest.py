from openpyxl import load_workbook

wb = load_workbook(filename = 'hello.xlsx')
sheet_ranges = wb['Sheet1']
print(sheet_ranges['A1'].value)

for sht in wb:
    print(sht['A1'].value)