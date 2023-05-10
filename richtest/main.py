from time import sleep

from rich.console import Console
from rich.progress import Progress
from rich.progress import BarColumn
from rich.progress import TextColumn
from rich.table import Column
from rich.table import Table

def table_test():
    table = Table()
    table.add_column('Count', justify='center')
    table.add_column('Case')
    table.add_column('Commands', justify='center')
    table.add_column('Comparison', justify='center')

    table.add_row('1', 'trainTunnelPar_0 (v18)', '2/3', '0/2')
    table.add_row('2', 'cylinderAdjustFlowUFixPar_0 (v12)', '7/7', '1/2')
    table.add_row('3', 'propellerPar_0 (v14)', '0/7', '0/2')
    table.add_row('4', 'cylinderAdjustFlowUFixPar_0 (v12)', '5/5', '2/2')

    console = Console()
    console.print(table)


def progress_test1():
    text_column = TextColumn("{task.description}", table_column=Column(ratio=1))
    bar_column = BarColumn(bar_width=None, table_column=Column(ratio=2))
    progress = Progress(text_column, bar_column, expand=True)

    with progress:
        for n in progress.track(range(100)):
            progress.print(n)
            sleep(0.1)


def progress_test2():
    with Progress() as progress:
        task1 = progress.add_task("[red]Downloading...", total=1000)
        task2 = progress.add_task("[green]Processing...", total=1000)
        task3 = progress.add_task("[cyan]Cooking...", total=1000)

        while not progress.finished:
            progress.update(task1, advance=0.5)
            progress.update(task2, advance=0.3)
            progress.update(task3, advance=0.9)
            sleep(0.02)


def color_test():
    console = Console(highlight=False)
    console.print('   #', end=' ', style='bright_white on black')
    console.print(f'{"Case":<40}', end=' ', style='bright_white on black')
    console.print(f'{"Commands":^10}', width=10, end=' ', style='bright_white on black')
    console.print(f'{"Compares":^10}', width=10, end=' ', style='bright_white on black')
    console.print('Status', width=20, justify='left', style='bright_white on black')

    console.print(f'{1:>4}', end=' ')
    console.print(f'{"trainTunnelPar_0":<40}', end=' ')
    console.print(f'{"0/5":^10}', end=' ')
    console.print(f'{"0/3":^10}', end=' ')
    console.print(f'{"Waiting":<20}')

    console.print(f'{2:>4}', end=' ')
    console.print(f'{"cylinderAdjustFlowUFixPar_0":<40}', end=' ')
    console.print('  ', style='black on white', end=''); console.print(' 2/5    ', end=' ')
    console.print(f'{"0/3":^10}', end=' ')
    console.print(f'{"Running":<20}', style='green')

    console.print(f'{3:>4}', end=' ')
    console.print(f'{"propellerPar_0":<40}', end=' ')
    console.print('   2/', style='black on white', end=''); console.print('4    ', end=' ')
    console.print(f'{"0/4":^10}', end=' ')
    console.print(f'{"Error":<20}', style='red')

    console.print(f'{4:>4}', end=' ')
    console.print(f'{"buildingWithWindowsAndSolarPar_0":<40}', end=' ')
    console.print(f'{"5/5":^10}', style='black on white', end=' ')
    console.print(f'{"4/4":^10}', end=' ')
    console.print(f'{"Failed":<20}', style='orange1')

if __name__ == '__main__':
    # progress_test2()
    color_test()
