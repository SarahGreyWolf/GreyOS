$in_files = Get-ChildItem -Path asm;
mkdir target;
mkdir target/asm;
mkdir target/elf;

echo "Assembling...";
echo "";
foreach ($file in $in_files) {
    $name_with_ext = $file.name;
    $name_without_ext = $name_with_ext.split('.')[0];
    echo "Assembling $name_with_ext";
    nasm "asm/$name_with_ext" -o "target/asm/$name_without_ext";
    echo "Assembly of $name_with_ext Complete";
    echo "";
}

echo "Building elf...";
echo "";
[System.Collections.ArrayList]$objects = @();
foreach ($file in $in_files) {
    $name_with_ext = $file.name;
    $name_without_ext = $name_with_ext.split('.')[0];
    echo "Assembling elf object for $name_with_ext";
    echo "";
    nasm -f elf64 "asm/$name_with_ext" -o "target/elf/$name_without_ext.o";
    $objects.Add("target/elf/$name_without_ext.o");
}
echo "Creating Kernel.bin from linker...";
ld -n -o target/kernel.bin -T linker.ld $objects;
echo "Complete"